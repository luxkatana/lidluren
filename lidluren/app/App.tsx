import { useState } from "react";
import { Text } from "react-native";
import { BottomNavigation } from "react-native-paper";
import { SafeAreaView } from "react-native-safe-area-context";
import Urenpage from "./UrenPage";

export default function App() {
	const [index, setindex] = useState(0);

	const [routes] = useState([
		{
			key: "uren",
			title: "Uren",
			focusedIcon: "heart",
			unfocusedIcon: "heart"
		},
	]);
	const renderScene = BottomNavigation.SceneMap({
		uren: Urenpage

	})
	return <>
		<BottomNavigation navigationState={{ index, routes }} onIndexChange={setindex} renderScene={renderScene} />



	</>
}
