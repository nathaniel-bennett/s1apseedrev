#![allow(non_camel_case_types)]

mod s1ap;
mod celevel;

use asn1_codecs::aper::AperCodec;
use std::os::raw::{c_char, c_uint};
use entropic::prelude::*;
use std::io::Write;


use rand::{rngs::StdRng, RngCore, SeedableRng};

fn main() {

    env_logger::init();

    let paths = std::fs::read_dir("./").unwrap();

    
    for path_res in paths {
        let path = path_res.unwrap().path();
        let bytes = std::fs::read(&path).unwrap();

        let mut aper_packet = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
        let Ok(pdu) = s1ap::S1AP_PDU::aper_decode(&mut aper_packet) else {
            println!("file {} failed to decode. Continuing...", path.to_str().unwrap());
            continue
        };

        let mut entropy = pdu.entropy::<DefaultEntropyScheme>();

        std::fs::write(format!("{}.structured", path.to_str().unwrap()), entropy.as_slice()).unwrap();

        let chk_pkt = s1ap::S1AP_PDU::from_entropy(&mut entropy.as_source());
        let mut chk_data = asn1_codecs::PerCodecData::new_aper();
        chk_pkt.aper_encode(&mut chk_data).unwrap();

        if (bytes.as_slice() != chk_data.into_bytes().as_slice()) {
            println!("WARNING: decode to reencode failed for {}", path.to_str().unwrap());
//            println!("{:?}", pdu);
//            println!("{:?}", chk_pkt);
        }
    }

    /*
    for i in 0..5000usize {
        println!("");

        rng.fill_bytes(&mut input);
        let mut filename: String;

        filename = format!("random_{}.aper", i);
        let im = s1ap::S1AP_PDU::from_entropy(&mut EntropySource::from_slice(input.as_slice()));
        let mut encoded = asn1_codecs::PerCodecData::new_aper();
        match im.aper_encode(&mut encoded) {
            Ok(()) => {
                let bytes = encoded.into_bytes();
                let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                filename = format!("{}_{}.aper", match im {
                    s1ap::S1AP_PDU::InitiatingMessage(i) => {
                        match i.value {
                            s1ap::InitiatingMessageValue::Id_CellTrafficTrace(_) => "cell_traffic_trace",
                            s1ap::InitiatingMessageValue::Id_ConnectionEstablishmentIndication(_) => "connection_establishment_indication",
                            s1ap::InitiatingMessageValue::Id_DeactivateTrace(_) => "deactivate_trace",
                            s1ap::InitiatingMessageValue::Id_DownlinkS1cdma2000tunnelling(_) =>"downlink_s1_cdma2000_tunnelling",
                            s1ap::InitiatingMessageValue::Id_E_RABModificationIndication(_) => "e_rab_modification_indication",
                            s1ap::InitiatingMessageValue::Id_E_RABModify(_) => "e_rab_modify",
                            s1ap::InitiatingMessageValue::Id_E_RABRelease(_) => "e_rab_release",
                            s1ap::InitiatingMessageValue::Id_E_RABReleaseIndication(_) => "e_rab_release_indication",
                            s1ap::InitiatingMessageValue::Id_E_RABSetup(_) => "e_rab_setup",
                            s1ap::InitiatingMessageValue::Id_ENBConfigurationUpdate(_) => "enb_configuration_update", // TODO
                            s1ap::InitiatingMessageValue::Id_ErrorIndication(_) => "error_indication",
                            s1ap::InitiatingMessageValue::Id_HandoverCancel(_) => "handover_cancel",
                            s1ap::InitiatingMessageValue::Id_HandoverNotification(_) => "handover_notification",
                            s1ap::InitiatingMessageValue::Id_HandoverPreparation(_) => "handover_preparation",
                            s1ap::InitiatingMessageValue::Id_HandoverResourceAllocation(_) => "handover_resource_allocation",
                            s1ap::InitiatingMessageValue::Id_HandoverSuccess(_) => "handover_success",
                            s1ap::InitiatingMessageValue::Id_InitialContextSetup(_) => "initial_context_setup",
                            s1ap::InitiatingMessageValue::Id_Kill(_) => "kill",
                            s1ap::InitiatingMessageValue::Id_LocationReport(_) => "location_report",
                            s1ap::InitiatingMessageValue::Id_LocationReportingControl(_) => "location_reporting_control",
                            s1ap::InitiatingMessageValue::Id_LocationReportingFailureIndication(_) => "location_reporting_failure_indication",
                            s1ap::InitiatingMessageValue::Id_MMECPRelocationIndication(_) => "mme_cp_relocation_indication",
                            s1ap::InitiatingMessageValue::Id_MMEConfigurationTransfer(_) => "mme_configuration_transfer", // TODO
                            s1ap::InitiatingMessageValue::Id_MMEConfigurationUpdate(_) => "mme_configuration_update",
                            s1ap::InitiatingMessageValue::Id_MMEDirectInformationTransfer(_) => "mme_direct_information_transfer",
                            s1ap::InitiatingMessageValue::Id_MMEEarlyStatusTransfer(_) => "mme_early_status_transfer",
                            s1ap::InitiatingMessageValue::Id_MMEStatusTransfer(_) => "mme_status_transfer", 
                            s1ap::InitiatingMessageValue::Id_NASDeliveryIndication(_) => "nas_delivery_indication", 
                            s1ap::InitiatingMessageValue::Id_NASNonDeliveryIndication(_) => "nas_non_delivery_indication",
                            s1ap::InitiatingMessageValue::Id_OverloadStart(_) => "overload_start",
                            s1ap::InitiatingMessageValue::Id_OverloadStop(_) => "overload_stop",
                            s1ap::InitiatingMessageValue::Id_PWSFailureIndication(_) => "pws_failure_indication",
                            s1ap::InitiatingMessageValue::Id_PWSRestartIndication(_) => "pws_restart_indication",
                            s1ap::InitiatingMessageValue::Id_Paging(_) => "paging",
                            s1ap::InitiatingMessageValue::Id_PathSwitchRequest(_) => "path_switch_request",
                            s1ap::InitiatingMessageValue::Id_RerouteNASRequest(_) => "reroute_nas_request",
                            s1ap::InitiatingMessageValue::Id_Reset(_) => "reset",
                            s1ap::InitiatingMessageValue::Id_RetrieveUEInformation(_) => "retrieve_ue_information",
                            s1ap::InitiatingMessageValue::Id_S1Setup(_) => "s1_setup",
                            s1ap::InitiatingMessageValue::Id_SecondaryRATDataUsageReport(_) => "secondary_rat_data_usage_report",
                            s1ap::InitiatingMessageValue::Id_TraceFailureIndication(_) => "trace_failure_indication",
                            s1ap::InitiatingMessageValue::Id_TraceStart(_) => "trace_restart",
                            s1ap::InitiatingMessageValue::Id_UECapabilityInfoIndication(_)    => "ue_capability_info_indication",
                            s1ap::InitiatingMessageValue::Id_UEContextModification(_) => "ue_context_modification",
                            s1ap::InitiatingMessageValue::Id_UEContextModificationIndication(_) => "ue_context_modification_indication",
                            s1ap::InitiatingMessageValue::Id_UEContextRelease(_) => "ue_context_release",
                            s1ap::InitiatingMessageValue::Id_UEContextReleaseRequest(_) => "ue_context_release_request",
                            s1ap::InitiatingMessageValue::Id_UEContextResume(_) => "ue_context_resume",
                            s1ap::InitiatingMessageValue::Id_UEContextSuspend(_) => "ue_context_suspend",
                            s1ap::InitiatingMessageValue::Id_UEInformationTransfer(_) => "ue_information_transfer",
                            s1ap::InitiatingMessageValue::Id_UERadioCapabilityIDMapping(_) => "ue_radio_capability_id_mapping",
                            s1ap::InitiatingMessageValue::Id_UERadioCapabilityMatch(_) => "ue_radio_capability_match",
                            s1ap::InitiatingMessageValue::Id_UplinkS1cdma2000tunnelling(_) => "uplink_s1_cdma_2000_tunnelling",
                            s1ap::InitiatingMessageValue::Id_WriteReplaceWarning(_) => "write_replace_warning",
                            s1ap::InitiatingMessageValue::Id_downlinkNASTransport(_) => "downlink_nas_transport",
                            s1ap::InitiatingMessageValue::Id_downlinkNonUEAssociatedLPPaTransport(_) => "downlink_non_ue_associated_lppa_transport",
                            s1ap::InitiatingMessageValue::Id_downlinkUEAssociatedLPPaTransport(_) => "downlink_ue_associated_lppa_transport",
                            s1ap::InitiatingMessageValue::Id_eNBCPRelocationIndication(_) => "enb_cp_relocation_indication",
                            s1ap::InitiatingMessageValue::Id_eNBConfigurationTransfer(_) => "enb_configuration_transfer",
                            s1ap::InitiatingMessageValue::Id_eNBDirectInformationTransfer(_) => "enb_direct_information_transfer",
                            s1ap::InitiatingMessageValue::Id_eNBEarlyStatusTransfer(_) => "enb_early_status_transfer",
                            s1ap::InitiatingMessageValue::Id_eNBStatusTransfer(_) => "enb_status_transfer", // TODO
                            s1ap::InitiatingMessageValue::Id_initialUEMessage(_) => "initial_ue_message",
                            s1ap::InitiatingMessageValue::Id_uplinkNASTransport(_) => "uplink_nas_transport",
                            s1ap::InitiatingMessageValue::Id_uplinkNonUEAssociatedLPPaTransport(_) => "uplink_non_ue_associated_lppa_transport",
                            s1ap::InitiatingMessageValue::Id_uplinkUEAssociatedLPPaTransport(_) => "uplink_ue_associated_lppa_transport",
                        }
                    }
                    s1ap::S1AP_PDU::SuccessfulOutcome(_) => "successful_outcome",
                    s1ap::S1AP_PDU::UnsuccessfulOutcome(_) => "unsuccessful_outcome",
                }, i);

                // std::fs::write(filename, bytes.as_slice()).unwrap();

                
                match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                    Ok(im_new) => {
                        std::fs::write(filename, bytes.as_slice()).unwrap();
                    },
                    Err(e) => {
                        println!("{} reencode failed: {}", filename, e);
                        // std::fs::write(filename, bytes.as_slice()).unwrap();
                    },
                }
                
            },
            Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
        }



        /*
        filename = format!("test{}.aper", i);
        match s1ap::E_RABReleaseIndication::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(17),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_S1Setup(s1ap::S1SetupRequest {
                        protocol_i_es: s1ap::S1SetupRequestProtocolIEs(vec! [
                            s1ap::S1SetupRequestProtocolIEs_Entry {
                                id: s1ap::ProtocolIE_ID(60),
                                criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                                value: s1ap::S1SetupRequestProtocolIEs_EntryValue::Id_eNBname(s1ap::ENBname("hello".to_string())),
                            }
                        ]),
                    }),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("e_rab_release_indication reencode failed: {}", e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create from arbitrary bytes for {}: {}", filename, e),
        };
        */

        /*
        filename = format!("e_rab_release_indication_{}.aper", i);
        match s1ap::E_RABReleaseIndication::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(8),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_E_RABReleaseIndication(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create from arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("e_rab_modification_indication_{}.aper", i);
        match s1ap::E_RABModificationIndication::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(50),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_E_RABModificationIndication(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("ue_context_release_request_{}.aper", i);
        match s1ap::UEContextReleaseRequest::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(18),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_UEContextReleaseRequest(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => { 

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("ue_context_modification_indication_{}.aper", i);
        match s1ap::UEContextModificationIndication::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(53),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_UEContextModificationIndication(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let mut bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("ue_context_suspend_request_{}.aper", i);
        match s1ap::UEContextSuspendRequest::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(55),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_UEContextSuspend(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let mut bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("ue_context_resume_request_{}.aper", i);
        match s1ap::UEContextResumeRequest::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(56),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_UEContextResume(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let mut bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("retrieve_ue_information_{}.aper", i);
        match s1ap::RetrieveUEInformation::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(58),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_RetrieveUEInformation(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let mut bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        filename = format!("enb_cp_relocation_indication_{}.aper", i);
        match s1ap::ENBCPRelocationIndication::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(60),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_eNBCPRelocationIndication(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let mut bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }

                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(e) => println!("Unable to create arbitrary bytes for {}: {}", filename, e),
        };

        

        filename = format!("s1_setup_request_{}.aper", i);
        match s1ap::S1SetupRequest::arbitrary(&mut Unstructured::new(input.as_slice())) {
            Ok(msg) => {
                let im = s1ap::S1AP_PDU::InitiatingMessage(s1ap::InitiatingMessage {
                    procedure_code: s1ap::ProcedureCode(17),
                    criticality: s1ap::Criticality(s1ap::Criticality::REJECT),
                    value: s1ap::InitiatingMessageValue::Id_S1Setup(msg),
                });

                let mut encoded = asn1_codecs::PerCodecData::new_aper();
                match im.aper_encode(&mut encoded) {
                    Ok(()) => {
                        let bytes = encoded.into_bytes();
                        let mut encoded = asn1_codecs::PerCodecData::from_slice_aper(bytes.as_slice());
                        match s1ap::S1AP_PDU::aper_decode(&mut encoded) {
                            Ok(_) => {

                            },
                            Err(e) => println!("{} reencode failed: {}", filename, e),
                        }
                        let mut f = std::fs::File::create(filename).unwrap();
                        f.write_all(bytes.as_slice()).unwrap();
                    },
                    Err(e) => println!("Unable to generate message from bytes for {}: {}", filename, e),
                }
            },
            Err(_) => println!("Unable to create arbitrary bytes for {}", filename),
        };
        */
    }
    */

}

