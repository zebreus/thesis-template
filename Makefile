PWD = $(shell pwd)
ASCIIDOCTOR     = asciidoctor -r asciidoctor-kroki
ASCIIDOCTOR_WEB_PDF = asciidoctor-web-pdf -r asciidoctor-kroki

TARGETS += $(TARGETS_WITHOUT_HTML) index.html
TARGETS_WITHOUT_HTML += $(PROCESSED_CHARTS) $(wildcard scripts/*.adoc)

.EXTRA_PREREQS:=Makefile
.PHONY: all pdf preview
all: paper.pdf
pdf: paper.pdf
preview: paper.css
	$(ASCIIDOCTOR_WEB_PDF) paper.adoc -o paper.pdf --preview

SCSS_FILES = $(wildcard styles/*.scss) $(wildcard styles/*/*.scss) $(wildcard styles/*/*/*.scss)

paper.css: $(SCSS_FILES)
	cd styles ; sass --update --sourcemap=none paper.scss:../paper.css

index.html: paper.adoc paper.css $(TARGETS_WITHOUT_HTML) 
	$(ASCIIDOCTOR) -S unsafe paper.adoc -o index.html

paper.pdf: paper.adoc paper.css $(TARGETS_WITHOUT_HTML) $(PROCESSED_CHARTS)
	$(ASCIIDOCTOR_WEB_PDF) paper.adoc -o paper.pdf

# ALL_DATA_JSON = experiments/all_data.json

# VEGA_CHART_FILES = $(shell find charts -name '*.json')
# VEGA_DATA_FILES = $(shell find data -type f -name '*.csv') data/minmax_overview.csv
# PROCESSED_CHARTS = $(addprefix processed-charts/,$(notdir $(VEGA_CHART_FILES)))

# process-charts: $(PROCESSED_CHARTS)

# $(PROCESSED_CHARTS) : processed-charts/%.vl.json : charts/%.vl.json $(VEGA_DATA_FILES)
# 	mkdir -p $(dir $@)
# 	bash process_chart.sh $< > $@
# 	touch $@

# data:
# 	cd experiments && make data
# 	make $(VEGA_DATA_FILES)

# $(ALL_DATA_JSON):
# 	cd experiments && make data

# $(VEGA_DATA_FILES) &: $(ALL_DATA_JSON) data/make_data.sh
# 	bash data/make_data.sh $< data

clean:
	rm -rf 

dist-clean: clean
	rm -rf $(TARGETS)

.PHONY: clean all dist-clean process-charts data