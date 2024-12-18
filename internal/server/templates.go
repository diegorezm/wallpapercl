package server

import (
	"context"
	"embed"
	"io"
	"text/template"
)

//go:embed views/*.html
var tmplFS embed.FS

type Template struct {
	templates *template.Template
}

func NewTemplateManager() *Template {
	funcMap := template.FuncMap{}

	templates := template.Must(template.New("").Funcs(funcMap).ParseFS(tmplFS, "views/*.html"))
	return &Template{
		templates: templates,
	}
}

func (t *Template) Render(w io.Writer, name string, data interface{}, c context.Context) error {
	return t.templates.ExecuteTemplate(w, name, data)
}
