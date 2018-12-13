import * as React from "react";
import "../../../filereader.js";
import {InputField} from "../util/util";
import * as $ from "jquery";
const styles = require("./main.css");

interface File {
    fileType: string,
    name: string,
}

export interface IMainProps {
    baseURL: string,
}

export interface IMainState {
    path: string[],
    listItems: File[],
}

export class Main extends React.Component<IMainProps, IMainState> {
    constructor (props: IMainProps) {
        super(props);

        this.state = {
            path: [],
            listItems: [],
        }

        // TODO add keylisteners to navigate more smoothly
        this.act({
            fileType: "folder",
            name: ""
        });
    }

    act (file: File) {
        $.ajax({
            url: this.props.baseURL+"/upload/"+file.name,
            type: "GET",
            contentType: "application/pdf",
            success: (data: any) =>  {
                this.applyData(data, file);
            },
            error: (e: any) => {
                this.doError(e);
            }
        });
    }

    applyData(data: any, file: File) {
        if (file.fileType == "file") {
            this.save(data, file);
        } else {
            try {
                const map = JSON.parse(data);

                if (map.token == "My Super Secret Token") {
                    const cs = map.folder.children;
                    let path = map.folder.name.split("/").slice(2);
                    if(! path[0]) {
                        path = [];
                    }
                    this.setState({
                        path: path,
                        listItems: cs
                    });
                } else {    // when you want to download a json file duhh
                    this.save(data, file);
                }
            } catch(e) {
                console.log(e);
                this.save(data, file);
            }
        }
    }

    save(data: any, file: File) {
        var fileData = new Blob([data]);
        if (window.navigator.msSaveOrOpenBlob) // IE10+
            window.navigator.msSaveOrOpenBlob(fileData, file.name);
        else { // Others
            var a = document.createElement("a"),
                    url = URL.createObjectURL(fileData);
            a.href = url;
            a.download = file.name;
            document.body.appendChild(a);
            a.click();
            setTimeout(function() {
                document.body.removeChild(a);
                window.URL.revokeObjectURL(url);  
            }, 0); 
        }
    }

    goBack () {
        let state = this.state;
        if (!state.path.pop()) {
            this.doError("couldn't go back, already in root");
        }
        this.setState(state);
        this.updateView();
    }

    doError(error: any) {
        console.error(error);
    }

    updateView() {
        this.act({
            fileType: "folder",
            name: this.state.path.join("/")
        });
    }

    newFolder(name: any) {
        console.log(name);
        let state = this.state;
        state.path.push(name);

        this.setState({
            path: state.path,
            listItems: []
        });
    }

    uploadFiles(files: FileList) {

        for(let i = 0; i < files.length; i++) {
            let file = files[i];

            let folder = this.props.baseURL+"/upload/"+this.state.path.join("/") + "/";
            $.ajax({
                url: folder+file.name,
                type: "POST",
                data: file,
                contentType: false,
                cache: false,
                processData:false,
                success: (data) => this.updateView(),
                error: (e) => this.doError(e)
            });
        }
    }

    openFileChooser() {
        const input: any = document.getElementById("file_chooser");
        input.click();
    }

    render () {
        const goTo = <InputField buttonName="Make!" action={(s) => this.newFolder(s)}/>;
        const listItems = this.state.listItems.sort((l1, l2) => {
            if (l1.fileType == l2.fileType) {
                return l1.name > l2.name ? 1 : -1;
            } else {
                return l1.fileType == "folder" ? -1 : 1;
            }
        });
        const items = listItems.map((i) => listItem(i, (i) => this.act(i)));
        return (
            <div className="FileExchanger Widget">
                <h1>File Exchanger</h1>
                <div className="Path">
                    <button onClick={(e) => this.goBack()} >Back</button>
                    <div>
                        <p>State: /{this.state.path.join("/")}</p>
                        {goTo}
                    </div>
                </div>
                <div className="FileChooser">
                    {items}
                    <div onClick={(e) => this.openFileChooser()}>
                        <span>+</span>
                    </div>
                </div>
                <input id="file_chooser" type="file" className="gone" onChange={e => this.uploadFiles(e.target.files)} multiple/>
            </div>
        );
    }
}

const listItem = (i: File, cb: (to: File) => void) => {
    return (
        <p 
            onClick={(e) => cb(i)} 
            key={i.name} className={i.fileType}>
            <span className="{i.fileType}-icon"/>
                {i.name}
        </p>);
}