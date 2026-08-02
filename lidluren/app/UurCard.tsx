import { Card, Divider, Text } from "react-native-paper";
import Timeline, { TimelineProps, Data } from 'react-native-timeline-flatlist'


interface UurCardProps {

	beginDate: Date,
	endDate: Date,

}
export default function UurCard(props: UurCardProps) {


	const data = [
		{ time: "09:00", title: "Begin", description: "Begin van werk" },
		{ time: "12:00", title: "Pauze begin (30 minuten)", description: "Pauze begin" },
		{ time: "12:30", title: "Pauze einde ", description: "Pauze einde" },
		{ time: "15:00", title: "Einde werk", description: "Hoera!" }
	];

	return <>
		<Card mode="outlined" style={{ margin: 10 }}>
			<Card.Title titleVariant="titleLarge" title={props.beginDate.toLocaleDateString("nl-NL", { year: "numeric", month: "long", weekday: "long", day: "numeric" })} />
			<Card.Content style={{ flex: 1, justifyContent: "center", alignItems: "center" }}>
				<Divider bold />
				<Text variant="bodyLarge">Begonnen om {props.beginDate.toLocaleTimeString("nl-NL", { hour: "numeric", minute: "numeric" })}</Text>
				<Text variant="bodyLarge">Geeindigd om {props.endDate.toLocaleTimeString("nl-NL", { hour: "numeric", minute: "numeric" })}</Text>



			</Card.Content>


		</Card>

	</>

}
