import ReactDOM from "react-dom/client";
import RouterProvider from "@/router";
import ThemeProvider from '@/theme'
import {getIndex} from '@/api'

console.debug(getIndex)

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <ThemeProvider>
    <RouterProvider />
  </ThemeProvider>
);
