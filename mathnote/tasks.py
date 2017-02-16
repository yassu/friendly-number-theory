from invoke import task

AUTHOR = 'yassu'
MAIN_BASENAME = 'main'
LATEX = 'platex'
DVIP = 'dvipdfmx'
VIEW_PDF = 'evince'


@task
def compile(ctx):
    """ コンパイルする """
    ctx.run('{} {}'.format(LATEX, MAIN_BASENAME + '.tex'))
    ctx.run('{} {}'.format(DVIP, MAIN_BASENAME + '.dvi'))


@task
def comp(ctx):
    """ コンパイルする """
    compile(ctx)


@task(compile)
def view(ctx):
    """ コンパイルして pdfファイルを開く """
    ctx.run('{} {}'.format(VIEW_PDF, MAIN_BASENAME + '.pdf'))


def get_junk_files():
    yield 'x.log'
    for filename in (MAIN_BASENAME + ext for ext in (
            '.aux', '.dvi', '.log', '.pdf')):
        yield filename


@task
def clean(ctx):
    """ コンパイル時にできたファイルを削除する """
    for filename in get_junk_files:
        ctx.run('rm {}'.format(filename))
