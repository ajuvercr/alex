import * as React from "react";
import * as fileSeach from "./fileSeach/Main";
import * as dairy from "./diary";
import * as eva from "./eva/Eva"
const fileSearch = require("./fileSeach/Main");
const styles = require("./hello.css");

export interface HelloProps { compiler: string; framework: string; }

export const Hello = (props: HelloProps) => (
    <div>
        <fileSeach.Main baseURL=""/>
        <hr></hr>
        <dairy.Dairy baseUrl=""/>
        <hr></hr>
        <eva.Dairy baseUrl=""/>
    </div>
);