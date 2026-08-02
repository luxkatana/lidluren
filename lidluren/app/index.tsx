import { PaperProvider, useTheme } from "react-native-paper";
import App from "./App";


export default function Home() {
	const theme = useTheme();
	return <>

		<PaperProvider theme={theme} >
			<App />


		</PaperProvider>

	</>
}
