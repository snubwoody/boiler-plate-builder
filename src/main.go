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
	
}

func cli(src_dir string, app_dir string){
	//TODO make the component list a list of checkboxes
	app := tview.NewApplication()
	
	var input *tview.InputField
	var list *tview.List
	var layout *tview.Flex
	var component_list *tview.List

	input = tview.NewInputField(). 
				SetLabel("Route name:").
				SetFieldWidth(10).
				SetDoneFunc(func(key tcell.Key) {
					create_route(input.GetText(), app_dir)
					//app.Stop()	
				})

	component_list = tview.NewList().
						AddItem("Navbar","",'1',func ()  {
							create_component("navbar",src_dir)
							app.Stop()
						}).
						AddItem("Button","",'2',func ()  {
							create_component("button",src_dir)
							app.Stop()
						})

	list = tview.NewList().
			AddItem("Generate component","",'1',func() {
				layout.AddItem(component_list,0,1,true)
				layout.RemoveItem(list)
			}).
			AddItem("Generate route","",'2',func ()  {
				layout.AddItem(input,0,1,true)
				layout.RemoveItem(list)		
			})
			
	//TODO not working properly in vs code terminal
	layout = tview.NewFlex().AddItem(list,0,1,true)
	
	layout.SetBorder(true).SetTitle("Boiler-plate builder")

	app.SetRoot(layout,true).Run()
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

