extern "C" {
	extern "C" typedef void (*Callback)(void* lua_State, void* ILuaInterface);

	void init();
	void newstate(Callback);
	void after_init(Callback);
	void before_init(Callback);
}