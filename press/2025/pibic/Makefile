# Makefile para compilação de relatório LaTeX
PDF_NAME = report.pdf
TEX_MAIN = main.tex
TEX_FILES = $(wildcard */*.tex) $(TEX_MAIN)
LATEX = lualatex
BIBTEX = bibtex
LATEX_OPTS = -interaction=nonstopmode

# Alvo padrão
all: $(PDF_NAME) view

# Forçar recreação do PDF sempre
.PHONY: $(PDF_NAME)

# Regra principal para construir o PDF
$(PDF_NAME): $(TEX_FILES)
	@echo "Compilando $(TEX_MAIN) para gerar $(PDF_NAME)..."
	-$(BIBTEX) $(basename $(TEX_MAIN))
	$(LATEX) $(LATEX_OPTS) $(TEX_MAIN)
	@cp -f main.pdf $(PDF_NAME)
	@echo "PDF compilado com sucesso: $(PDF_NAME)"

# Limpeza de arquivos temporários
clean:
	@echo "Removendo arquivos temporários..."
	@rm -f *.aux *.log *.toc *.lof *.lot *.out *.blg *.bbl *.dvi *.synctex.gz */*.aux main.pdf
	@echo "Arquivos temporários removidos."

# Remoção completa (incluindo PDFs)
distclean: clean
	@rm -f $(PDF_NAME)
	@echo "Todos os arquivos gerados foram removidos."

# Compilação rápida (uma passagem apenas)
quick:
	@echo "Compilação rápida..."
	$(LATEX) $(LATEX_OPTS) $(TEX_MAIN)
	@cp -f main.pdf $(PDF_NAME)
	@echo "Compilação rápida concluída. Referências podem estar desatualizadas."

# Visualização do PDF
view: $(PDF_NAME)
	@(xdg-open $(PDF_NAME) || open $(PDF_NAME) || start $(PDF_NAME)) 2>/dev/null

.PHONY: all clean distclean quick view
