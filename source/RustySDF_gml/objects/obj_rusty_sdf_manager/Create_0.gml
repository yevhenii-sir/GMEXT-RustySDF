if (instance_number(object_index) > 1) {
    instance_destroy(); 
    exit;
}

RustySDF_Init();
RustySDF_AtlasInit(1024, 1024, 4);

global.rusty_sdf_async_manager = new RustySDF_AsyncManager();