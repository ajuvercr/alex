import * as React from "react";

export interface IFieldProps {
    action: (state: string) => void,
    name?: string,
    buttonName?: string,
}

export interface IFieldState {
    value: string,
}


export class InputField extends React.Component<IFieldProps, IFieldState> {
    constructor (props: IFieldProps) {
        super(props);

        this.state = {
            value: "",
        }

        this.setState.bind(this);
        this.render.bind(this);
    }

    update(event: any) {
        this.setState({
            value: event.target.value
        });
    }

    render() {
        const textField = <input value={this.state.value} onChange={e => this.update(e)}/>;
        const button = <button onClick={(e) => this.props.action(this.state.value)}>{this.props.buttonName}</button>
        return (
            <div>
                <p>{this.props.name}</p>
                {textField}
                {button}
            </div>
        );
    }
}