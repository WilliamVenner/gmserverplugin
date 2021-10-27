#pragma once

#include <plugin.h>

GLuaPackPlugin::GLuaPackPlugin() {}

static GLuaPackPlugin* INSTANCE = new GLuaPackPlugin();
uintptr_t CreateInterface() {
	return (uintptr_t) INSTANCE;
}