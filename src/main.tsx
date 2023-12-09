import ReactDOM from "react-dom/client";
import Router from "@/router";
import Theme from '@/theme'

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(<Theme><Router /></Theme>);
