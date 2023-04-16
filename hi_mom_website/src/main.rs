// This is an AI code generation experiment.
// I prompted ChatGPT (GPT-4) to create a website that displays "hi mom",
// along with a button to randomly toggle the text color.
// There is a 20% chance of getting a fireworks animation everytime the color is toggled.
//
// Note: I didn't write a single line of code myself.
//       I simply sent the compilation errors back to ChatGPT to fix it.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

async fn hi_mom() -> impl Responder {
    let mut rng = thread_rng();
    let color_range = Uniform::new_inclusive(0, 16777215);
    let random_color = rng.sample(color_range);
    let random_color_hex = format!("{:06x}", random_color);

    HttpResponse::Ok().content_type("text/html").body(format!(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    background-color: #f5f5f5;
                    position: relative;
                    overflow: hidden;
                }}

                .container {{
                    text-align: center;
                    padding: 20px;
                    background-color: #ffffff;
                    border-radius: 10px;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }}

                button {{
                    font-family: Arial, sans-serif;
                    font-size: 1rem;
                    padding: 10px 20px;
                    background-color: #4CAF50;
                    color: white;
                    border: none;
                    border-radius: 5px;
                    cursor: pointer;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }}

                button:hover {{
                    background-color: #45a049;
                }}

                .firework {{
                    position: absolute;
                    pointer-events: none;
                    animation: firework 0.8s linear forwards;
                }}

                .firework::before,
                .firework::after {{
                    content: "";
                    display: block;
                    width: 4px;
                    height: 4px;
                    background: currentColor;
                    position: absolute;
                    border-radius: 50%;
                    animation: explosion 1s ease-out infinite;
                }}

                @keyframes firework {{
                    0% {{
                        transform: translateY(100%);
                    }}
                    100% {{
                        transform: translateY(calc(50% - 30px));
                    }}
                }}

                @keyframes explosion {{
                    0% {{
                        box-shadow: 0 0, 0 0, 0 0, 0 0, 0 0, 0 0, 0 0, 0 0;
                        transform: scale(0.1);
                        opacity: 1;
                    }}
                    100% {{
                        box-shadow: 0 50px, 50px 0, 0 -50px, -50px 0, 50px 50px, -50px -50px, -50px 50px, 50px -50px;
                        transform: scale(1);
                        opacity: 0;
                    }}
                }}
            </style>

            <script>
                function createFirework(color) {{
                    var firework = document.createElement("div");
                    firework.className = "firework";
                    firework.style.color = color;
                    firework.style.left = Math.random() * 100 + "vw";
                    document.body.appendChild(firework);
                    setTimeout(() => {{
                      document.body.removeChild(firework);
                    }}, 1800);
                }}

                function changeTextColor() {{
                  var colors = ["red", "blue", "green", "yellow", "purple", "orange"];
                  var randomColor = colors[Math.floor(Math.random() * colors.length)];
                  document.getElementById("greeting").style.color = randomColor;
                  if (Math.random() < 0.2) {{
                    createFirework(randomColor);
                  }}
                }}
            </script>
        </head>
        <body>
            <div class="container">
                <h1 id="greeting" style="color: #{};">Hi mom</h1>
                <button onclick="changeTextColor()">Toggle Color</button>
            </div>
        </body>
        </html>
    "#,
        random_color_hex
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hi_mom)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
