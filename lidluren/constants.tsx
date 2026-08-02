import axios from "axios";
import { Platform } from "react-native";

export const axiosClient = axios.create({
	baseURL: Platform.OS == "web" ? "http://127.0.0.1:8000" : "http://192.168.1.45:8000"
	, headers: {
		"Content-Type": "application/json",
		"Accept": "application/json"
	}
});
