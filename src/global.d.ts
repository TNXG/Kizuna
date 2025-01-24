declare global {
	// build_media_update 函数返回的媒体更新数据结构
	interface MediaUpdate {
		title: string; // 媒体标题
		artist: string; // 艺术家或创作者
		SourceAppName: string; // 来源应用名称
	}

	// build_data 函数返回的数据结构
	interface UpdateData {
		timestamp: number; // 时间戳，单位为秒
		process: string; // 进程名称
		window_name?: string; // 窗口名称
		media?: MediaUpdate; // 媒体信息（可选字段，如果没有标题，则不存在该字段）
	}

	interface ReturnData {
		data: UpdateData;
		icon?: string;
	}
}

export { };
