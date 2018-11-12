import * as React from "react";
import * as fileSeach from "./fileSeach/Main";
import * as dairy from "./diary";
const fileSearch = require("./fileSeach/Main");
const styles = require("./hello.css");

export interface HelloProps { compiler: string; framework: string; }

export const Hello = (props: HelloProps) => (
    <div>
        <fileSeach.Main baseURL=""/>
        <dairy.Dairy baseUrl=""/>
    </div>
);