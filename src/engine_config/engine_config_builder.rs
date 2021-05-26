use std::ffi::CString;

use crate::{Algorithm, Boolean, Osrm};

use super::c_engine_config::CEngineConfig;

pub struct EngineConfigBuilder<'a> {
    storage_config: &'a str,
    max_locations_trip: i32,
    max_locations_viaroute: i32,
    max_locations_distance_table: i32,
    max_locations_map_matching: i32,
    max_radius_map_matching: f64,
    max_results_nearest: i32,
    max_alternatives: i32,
    use_shared_memory: bool,
    memory_file: Option<&'a str>,
    use_mmap: bool,
    algorithm: Algorithm,
    verbosity: Option<&'a str>,
    dataset_name: Option<&'a str>,
}

impl<'a> EngineConfigBuilder<'a> {
    pub fn new(storage_config: &str) -> EngineConfigBuilder {
        EngineConfigBuilder {
            storage_config: storage_config,
            max_locations_trip: -1,
            max_locations_viaroute: -1,
            max_locations_distance_table: -1,
            max_locations_map_matching: -1,
            max_radius_map_matching: -1.0,
            max_results_nearest: -1,
            max_alternatives: 3,
            use_shared_memory: true,
            memory_file: None,
            use_mmap: true,
            algorithm: Algorithm::CH,
            verbosity: None,
            dataset_name: None,
        }
    }

    pub fn set_storate_config<'i>(
        &'a mut self,
        storage_config: &'a str,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.storage_config = storage_config;
        self
    }

    pub fn set_max_locations_trip<'i>(
        &'a mut self,
        max_locations_trip: i32,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_locations_trip = max_locations_trip;
        self
    }

    pub fn set_max_locations_viaroute<'i>(
        &'a mut self,
        max_locations_viaroute: i32,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_locations_viaroute = max_locations_viaroute;
        self
    }

    pub fn set_max_locations_distance_table<'i>(
        &'a mut self,
        max_locations_distance_table: i32,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_locations_distance_table = max_locations_distance_table;
        self
    }

    pub fn set_max_locations_map_matching<'i>(
        &'a mut self,
        max_locations_map_matching: i32,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_locations_map_matching = max_locations_map_matching;
        self
    }

    pub fn set_max_radius_map_matching<'i>(
        &'a mut self,
        max_radius_map_matching: f64,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_radius_map_matching = max_radius_map_matching;
        self
    }

    pub fn set_max_results_nearest<'i>(
        &'a mut self,
        max_results_nearest: i32,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_results_nearest = max_results_nearest;
        self
    }

    pub fn set_max_alternatives<'i>(
        &'a mut self,
        max_alternatives: i32,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.max_alternatives = max_alternatives;
        self
    }

    pub fn set_use_shared_memory<'i>(
        &'a mut self,
        use_shared_memory: bool,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.use_shared_memory = use_shared_memory;
        self
    }

    pub fn set_memory_file<'i>(
        &'a mut self,
        memory_file: Option<&'a str>,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.memory_file = memory_file;
        self
    }

    pub fn set_use_mmap<'i>(&'a mut self, use_mmap: bool) -> &'i mut EngineConfigBuilder<'a> {
        self.use_mmap = use_mmap;
        self
    }

    pub fn set_algorithm<'i>(
        &'a mut self,
        algorithm: Algorithm,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.algorithm = algorithm;
        self
    }

    pub fn set_verbosity<'i>(
        &'a mut self,
        verbosity: Option<&'a str>,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.verbosity = verbosity;
        self
    }

    pub fn set_dataset_name<'i>(
        &'a mut self,
        dataset_name: Option<&'a str>,
    ) -> &'i mut EngineConfigBuilder<'a> {
        self.dataset_name = dataset_name;
        self
    }

    pub fn build(&self) -> Result<Osrm, String> {
        let c_storage_config = CString::new(self.storage_config.clone()).unwrap();
        let mut c_engine_config = CEngineConfig::new(&c_storage_config);

        c_engine_config.max_alternatives = self.max_alternatives;
        c_engine_config.max_locations_viaroute = self.max_locations_viaroute;
        c_engine_config.max_locations_distance_table = self.max_locations_distance_table;
        c_engine_config.max_locations_map_matching = self.max_locations_map_matching;
        c_engine_config.max_radius_map_matching = self.max_radius_map_matching;
        c_engine_config.max_results_nearest = self.max_results_nearest;

        if self.use_shared_memory {
            c_engine_config.use_shared_memory = Boolean::TRUE;
        } else {
            c_engine_config.use_shared_memory = Boolean::FALSE;
        }

        match &self.memory_file {
            Some(memory_file_string) => {
                let c_memory_file = CString::new(memory_file_string.clone()).unwrap();

                c_engine_config.memory_file = c_memory_file.as_ptr();
            }
            None => {
            }
        }

        if self.use_mmap {
            c_engine_config.use_mmap = Boolean::TRUE;
        } else {
            c_engine_config.use_mmap = Boolean::FALSE;
        }

        c_engine_config.algorithm = self.algorithm.clone();

        match &self.verbosity {
            Some(verbosity_string) => {
                let c_verbosity = CString::new(verbosity_string.clone()).unwrap();

                c_engine_config.verbosity = c_verbosity.as_ptr();
            }
            None => {}
        }

        match &self.dataset_name {
            Some(dataset_name_string) => {
                let c_dataset_name = CString::new(dataset_name_string.clone())
                    .expect("to_CEngineConfig::dataset_name::new failed");

                c_engine_config.dataset_name = c_dataset_name.as_ptr();
            }
            None => {}
        }

        Osrm::new(c_engine_config)
    }
}