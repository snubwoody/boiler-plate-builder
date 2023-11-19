package main

import (
	"fmt"
	"os"
	"path"

	"github.com/gdamore/tcell/v2"
	"github.com/rivo/tview"
)


func main() {
	
	root_dir := path.Dir("../next-app/")
	src_dir := path.Join(root_dir,"src")
	app_dir := path.Join(src_dir,"app")		
		
	cli(src_dir,app_dir)
	//create_route("login",app_dir)
	
}

func cli(src_dir string, app_dir string){
	app := tview.NewApplication()

	var input *tview.InputField

	input = tview.NewInputField(). 
				SetLabel("Route name:").
				SetFieldWidth(10).
				SetDoneFunc(func(key tcell.Key) {
					create_route(input.GetText(),app_dir)
					app.Stop()
				})

	component_list := tview.NewList().
						AddItem("Navbar","",'1',func ()  {
							create_component("navbar",src_dir)
						}).
						AddItem("Button","",'2',func ()  {
							create_component("button",src_dir)
						})

	list := tview.NewList().
			AddItem("Generate component","",'1',func() {
				app.SetRoot(component_list,true)
			}).
			AddItem("Generate route","",'2',func ()  {
				app.SetRoot(input,true)
				
			})

	app.SetRoot(list,true).Run()
}

func create_component(name string,src_dir string){
	
	err := os.WriteFile(path.Join(src_dir,fmt.Sprintf("components/%s.tsx",name)),[]byte("Hello"),0666)

	if err != nil {
		println(err)
	}
}

func create_route(name string,app_dir string){
	err := os.Mkdir(path.Join(app_dir,name),0750)

	if err != nil {
		println(err)
	}

	route_dir := path.Join(app_dir,name)

	file_err := os.WriteFile(path.Join(route_dir,"page.tsx"),[]byte(""),0666)

	if file_err != nil {
		println(file_err)
	}
}

