import * as React from "react";
import * as $ from "jquery";
import {InputField} from "./util/util";

interface IDairyProps {
    baseUrl: string
}

interface IDairyState {
    title: string,
    content: string,
}

export class Dairy extends React.Component<IDairyProps, IDairyState> {
    constructor(props: IDairyProps) {
        super(props);
        this.state = {
            title: "",
            content: ""
        }
    }

    update(event: any, key: string) {
        if(key == "title") {
            this.setState({
                title: event.target.value,
                content: this.state.content
            });
        }else{
            this.setState({
                title: this.state.title,
                content: event.target.value,
            });
        }
    }

    doDairy() {
        const data = {
            title: this.state.title,
            content: this.state.content,
        }

        $.ajax({
            url: this.props.baseUrl+"/dairy",
            type: "POST",
            data: data,
            contentType: "application/json",
            cache: false,
            processData:false,
            success: (data) => {
                this.setState({
                    content: "",
                    title: ""
                });
                console.log(data);
            },
            error: function(e) {
                console.log(e);
            }
        });
    }

    render() {
        return (
            <div>
                <p>Title</p>
                <input key="title" type="text" placeholder="Title" value={this.state.title} onChange={(e) => this.update(e, "title")}/>

                <p>Content</p>
                <input key="content" type="text" placeholder="Content" value={this.state.content} onChange={(e) => this.update(e, "content")}/>
                <button onClick={e => this.doDairy()}>Send!</button>
            </div>
        );
    }
}