export function name() {
    console.log("Works")
    alert("workinng")
    return "Rust"
}

export class MyClass{
    constructor(){
        this._number = 43;
    }
    get number(){
        return this._number       
    }
    set number(arg){
        this._number = arg
    }
    render(){
        return `Rendered ${this.number}`
    }
}