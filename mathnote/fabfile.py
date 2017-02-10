#!/usr/bin/env python
# coding: UTF-8
import os
import subprocess
from fabric.api import local, task

AUTHOR = 'yassu'
MAIN_BASENAME = 'main'
LATEX = 'platex'
DVIP = 'dvipdfmx'
VIEW_PDF = 'evince'


class Commands(list):
    def run(self):
        for cmd in self:
            print(cmd)
            local(cmd)


@task
def compile(basename=MAIN_BASENAME):
    " latex and dvip "

    cmds = Commands()
    cmds.append('%s %s' % (LATEX, basename + '.tex'))
    cmds.append('%s %s' % (DVIP, basename + '.dvi'))
    cmds.run()


@task(alias='compile')
def comp(basename=MAIN_BASENAME):
    compile(basename)


@task
def view(basename=MAIN_BASENAME, make=True):
    if make:
        compile()

    cmds = Commands()
    cmds.append('%s %s' % (VIEW_PDF, basename + '.pdf'))
    cmds.run()

def is_using_git():
    try:
        subprocess.check_output(['git', '--version'])
    except FileNotFoundError:
        return False

    return subprocess.check_output(['ls', '.git']) == '\n'


@task
def submit(basename=MAIN_BASENAME, make=True, tag=None, author=AUTHOR):

    if is_using_git() and tag is None:
        tags = subprocess.check_output(['git', 'tag']).split('\n')
        tag = tags[-2] if len(tags) >= 2 else None

    out_basename = '%s-%s' % (AUTHOR, basename)
    if tag is not None:
        out_basename += '-%s' % (tag)

    if make:
        compile()

    local('cp %s %s' % (basename + '.pdf', out_basename + '.pdf'))


@task(alias='compile')
def hand(basename=MAIN_BASENAME, make=True, tag=None, author=AUTHOR):
    submit(basename, make, tag, author)

def get_cleaned_filenames(base_filename):
    yield 'fabfile.pyc'
    yield 'x.log'
    for filename in (base_filename + ext for ext in
            ('.aux', '.dvi', '.log', '.pdf')):
        yield filename

@task
def clean(basename=MAIN_BASENAME):
    for filename in get_cleaned_filenames(MAIN_BASENAME):
        if os.path.isfile(filename):
            os.remove(filename)
