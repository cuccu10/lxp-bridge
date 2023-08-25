mod common;
use common::*;

#[tokio::test]
async fn for_param() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::ReadParam {
        datalog: inverter.datalog(),
        register: 0,
        values: vec![1, 0],
    };

    assert_eq!(
        mqtt::Message::for_param(packet).unwrap(),
        vec![mqtt::Message {
            topic: "2222222222/param/0".to_owned(),
            retain: true,
            payload: "1".to_owned()
        }]
    );
}

#[tokio::test]
async fn for_hold_single() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog(),
        device_function: lxp::packet::DeviceFunction::ReadHold,
        inverter: inverter.serial(),
        register: 0,
        values: vec![1, 0],
    };

    assert_eq!(
        mqtt::Message::for_hold(packet).unwrap(),
        vec![mqtt::Message {
            topic: "2222222222/hold/0".to_owned(),
            retain: true,
            payload: "1".to_owned()
        }]
    );
}

#[tokio::test]
async fn for_hold_21() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog(),
        device_function: lxp::packet::DeviceFunction::ReadHold,
        inverter: inverter.serial(),
        register: 21,
        values: vec![12, 34],
    };

    assert_eq!(
        mqtt::Message::for_hold(packet).unwrap(),
        vec![mqtt::Message { topic: "2222222222/hold/21".to_owned(), retain: true, payload: "8716".to_owned() },
             mqtt::Message { topic: "2222222222/hold/21/bits".to_owned(), retain: true, payload: "{\"eps_en\":\"OFF\",\"ovf_load_derate_en\":\"OFF\",\"drms_en\":\"ON\",\"lvrt_en\":\"ON\",\"anti_island_en\":\"OFF\",\"neutral_detect_en\":\"OFF\",\"grid_on_power_ss_en\":\"OFF\",\"ac_charge_en\":\"OFF\",\"sw_seamless_en\":\"OFF\",\"set_to_standby\":\"ON\",\"forced_discharge_en\":\"OFF\",\"charge_priority_en\":\"OFF\",\"iso_en\":\"OFF\",\"gfci_en\":\"ON\",\"dci_en\":\"OFF\",\"feed_in_grid_en\":\"OFF\"}".to_owned() }
        ]
    );
}

#[tokio::test]
async fn for_hold_110() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog(),
        device_function: lxp::packet::DeviceFunction::ReadHold,
        inverter: inverter.serial(),
        register: 110,
        values: vec![9, 4],
    };

    assert_eq!(
        mqtt::Message::for_hold(packet).unwrap(),
        vec![mqtt::Message { topic: "2222222222/hold/110".to_owned(), retain: true, payload: "1033".to_owned() },
             mqtt::Message { topic: "2222222222/hold/110/bits".to_owned(), retain: true, payload: "{\"ub_pv_grid_off_en\":\"ON\",\"ub_run_without_grid\":\"OFF\",\"ub_micro_grid_en\":\"OFF\"}".to_owned() }
        ]
    );
}

#[tokio::test]
async fn for_hold_multi() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog,
        device_function: lxp::packet::DeviceFunction::ReadHold,
        inverter: inverter.serial,
        register: 12,
        values: vec![22, 6, 7, 8, 9, 0],
    };

    assert_eq!(
        mqtt::Message::for_hold(packet).unwrap(),
        vec![
            mqtt::Message {
                topic: "2222222222/hold/12".to_owned(),
                retain: true,
                payload: "1558".to_owned()
            },
            mqtt::Message {
                topic: "2222222222/hold/13".to_owned(),
                retain: true,
                payload: "2055".to_owned()
            },
            mqtt::Message {
                topic: "2222222222/hold/14".to_owned(),
                retain: true,
                payload: "9".to_owned()
            },
        ]
    );
}

#[tokio::test]
#[cfg_attr(not(feature = "mocks"), ignore)]
async fn for_input() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog,
        device_function: lxp::packet::DeviceFunction::ReadInput,
        inverter: inverter.serial,
        register: 0,
        values: [0; 80].to_vec(),
    };

    assert_eq!(
        mqtt::Message::for_input(packet, false).unwrap(),
        vec![mqtt::Message {
            topic: "2222222222/inputs/1".to_owned(),
            retain: false,
            payload: "{\"status\":0,\"v_pv_1\":0.0,\"v_pv_2\":0.0,\"v_pv_3\":0.0,\"v_bat\":0.0,\"soc\":0,\"soh\":0,\"internal_fault\":0,\"p_pv\":0,\"p_pv_1\":0,\"p_pv_2\":0,\"p_pv_3\":0,\"p_battery\":0,\"p_charge\":0,\"p_discharge\":0,\"v_ac_r\":0.0,\"v_ac_s\":0.0,\"v_ac_t\":0.0,\"f_ac\":0.0,\"p_inv\":0,\"p_rec\":0,\"pf\":0.0,\"v_eps_r\":0.0,\"v_eps_s\":0.0,\"v_eps_t\":0.0,\"f_eps\":0.0,\"p_eps\":0,\"s_eps\":0,\"p_grid\":0,\"p_to_grid\":0,\"p_to_user\":0,\"e_pv_day\":0.0,\"e_pv_day_1\":0.0,\"e_pv_day_2\":0.0,\"e_pv_day_3\":0.0,\"e_inv_day\":0.0,\"e_rec_day\":0.0,\"e_chg_day\":0.0,\"e_dischg_day\":0.0,\"e_eps_day\":0.0,\"e_to_grid_day\":0.0,\"e_to_user_day\":0.0,\"v_bus_1\":0.0,\"v_bus_2\":0.0,\"time\":1646370367,\"datalog\":\"2222222222\"}".to_owned()
        }]
    );

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog,
        device_function: lxp::packet::DeviceFunction::ReadInput,
        inverter: inverter.serial,
        register: 0,
        values: [0; 4].to_vec(),
    };

    assert_eq!(
        mqtt::Message::for_input(packet, true).unwrap(),
        vec![
            mqtt::Message {
                topic: "2222222222/input/0".to_owned(),
                retain: false,
                payload: "0".to_owned()
            },
            mqtt::Message {
                topic: "2222222222/input/0/parsed".to_owned(),
                retain: false,
                payload: "Standby".to_owned()
            },
            mqtt::Message {
                topic: "2222222222/input/1".to_owned(),
                retain: false,
                payload: "0".to_owned()
            }
        ]
    );
}

#[tokio::test]
async fn for_input_ignore_127_254() {
    common_setup();

    let inverter = Factory::inverter();

    let packet = lxp::packet::TranslatedData {
        datalog: inverter.datalog(),
        device_function: lxp::packet::DeviceFunction::ReadInput,
        inverter: inverter.serial(),
        register: 127,
        values: [0; 254].to_vec(),
    };

    assert_eq!(mqtt::Message::for_input(packet, false).unwrap(), vec![]);
}
