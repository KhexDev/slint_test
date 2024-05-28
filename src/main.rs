slint::slint! {
    import { VerticalBox, Button, HorizontalBox, AboutSlint, Switch, CheckBox } from "std-widgets.slint";

    export enum Operation {
        Addition,
        Substraction,
        Multiplication,
        Division
    }

    export global Calcul {
        in property <string> a;
        in property <string> b;
        in property <string> result;
        in property <Operation> operation;

        callback calculate();
    }

export component App inherits Window {
    title: "Slint Test!";
    width: 640px;
    height: 480px;


    VerticalBox {
        HorizontalBox {
            Rectangle {
                background: gray;
                TextInput {
                    text: "A";
                    edited => {
                        Calcul.a = self.text;
                    }
                }
            }
            Rectangle {
                background: gray;
                TextInput {
                    text: "B";
                    edited => {
                    Calcul.b = self.text;
                    }
                }
            }
        }
        HorizontalBox {
            CheckBox {text: "Addition"; toggled => {
                Calcul.operation = Operation.Addition;
                debug("Addition");
            }}
            CheckBox {text: "Substraction"; toggled => {
                Calcul.operation = Operation.Substraction;
                debug("Substraction");
            }}
            CheckBox {text: "Multiplication"; toggled => {
                Calcul.operation = Operation.Multiplication;
                debug("Multiplication");
            }}
            CheckBox {text: "Division"; toggled => {
                Calcul.operation = Operation.Division;
                debug("Division");
            }}
        }
        Button {text: "Calculate"; clicked => {
            Calcul.calculate();
        }}
        Rectangle {
            background: gray;
            Text {text: "Resultat:" + Calcul.result;}
        }
    }
}
}
fn main() {
    println!("running app");
    let app = App::new().unwrap();
    let calcul = app.global::<Calcul>();

    let weak = app.as_weak();
    calcul.on_calculate(move || {
        let app = weak.upgrade().unwrap();
        let calcul = app.global::<Calcul>();
        let a = calcul.get_a().parse::<f32>().expect("Failed to parse A");
        let b = calcul.get_b().parse::<f32>().expect("Failed to parse B");

        match calcul.get_operation() {
            Operation::Addition => {
                calcul.set_result(format!("{}", a + b).into());
            }
            Operation::Substraction => {
                calcul.set_result(format!("{}", a - b).into());
            }
            Operation::Multiplication => {
                calcul.set_result(format!("{}", a * b).into());
            }
            Operation::Division => {
                calcul.set_result(format!("{}", a / b).into());
            }
        }
    });

    let _ = app.run();
}
