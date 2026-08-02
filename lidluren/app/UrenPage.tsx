import { FlatList, View } from "react-native";
import UurCard from "./UurCard";
import { SafeAreaView } from "react-native-safe-area-context";
import { axiosClient } from "../constants";
import { useTheme, Text, ActivityIndicator } from "react-native-paper";
import { useEffect, useState } from "react";
export default function UrenPage() {
	const [shifts, setshifts] = useState();
	const [loading, setloading] = useState(true);
	useEffect(() => {

		const fetchdata = async () => {
			const response = await axiosClient.get("/shifts");
			setloading(false);
			setshifts(response.data);

		}
		fetchdata()



	}, []);
	return <>
		<SafeAreaView style={{ justifyContent: "center", flex: 1, alignItems: "center" }}>
			{loading ? (
				<>
					<ActivityIndicator animating size="large" />
					<Text>Importing shifts</Text>
				</>
			) : <>
				<FlatList data={shifts} renderItem={({ item }) => <UurCard beginDate={new Date(item.begintime)} endDate={new Date(item.endtime)} />}
					keyExtractor={(item) => item.shiftid} />

			</>}
		</SafeAreaView>



	</>
}
