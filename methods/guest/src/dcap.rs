mod quote;
mod tcb_info;
mod constants;
// TODO: need port ring
// mod utils;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::dcap::quote::{AuthData, EnclaveReport, Quote};
use crate::dcap::tcb_info::TcbInfo;
use crate::dcap::constants::*;
// TODO: need port ring
// use crate::dcap::utils::*;
use crate::Error;

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct SgxV30QuoteCollateral {
    pub pck_crl_issuer_chain: String,
    pub root_ca_crl: String,
    pub pck_crl: String,
    pub tcb_info_issuer_chain: String,
    pub tcb_info: String,
    pub tcb_info_signature: Vec<u8>,
    pub qe_identity_issuer_chain: String,
    pub qe_identity: String,
    pub qe_identity_signature: Vec<u8>,
}

#[allow(clippy::type_complexity)]
pub fn verify(
    raw_quote: &[u8],
    quote_collateral: &SgxV30QuoteCollateral,
    now: u64,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, u16, u16, String, Vec<String>), Error> {
    // Parse data

    let mut quote = raw_quote;
    let quote = Quote::decode(&mut quote).map_err(|_| Error::CodecError)?;

    let tcb_info = pink_json::from_str::<TcbInfo>(&quote_collateral.tcb_info)
        .map_err(|_| Error::CodecError)?;

    let next_update =
        chrono::DateTime::parse_from_rfc3339(&tcb_info.next_update).map_err(|_| Error::CodecError)?;
    if now > next_update.timestamp() as u64 {
        return Err(Error::TCBInfoExpired);
    }

    let _now_in_milli = now * 1000;

    // Verify enclave

    // Seems we verify MR_ENCLAVE and MR_SIGNER is enough
    // skip verify_misc_select_field
    // skip verify_attributes_field

    // Verify integrity

    // TODO: need port ring
    // Check TCB info cert chain and signature
    // let leaf_certs = extract_certs(quote_collateral.tcb_info_issuer_chain.as_bytes())?;
    // if leaf_certs.len() < 2 {
    //     return Err(Error::CertificateChainIsTooShort);
    // }
    // let leaf_cert: webpki::EndEntityCert = webpki::EndEntityCert::try_from(&leaf_certs[0])
    //     .map_err(|_| Error::LeafCertificateParsingError)?;
    // let intermediate_certs = &leaf_certs[1..];
    // verify_certificate_chain(&leaf_cert, intermediate_certs, now_in_milli)?;
    // let asn1_signature = encode_as_der(&quote_collateral.tcb_info_signature)?;
    // if leaf_cert
    //     .verify_signature(
    //         webpki::ECDSA_P256_SHA256,
    //         quote_collateral.tcb_info.as_bytes(),
    //         &asn1_signature,
    //     )
    //     .is_err()
    // {
    //     return Err(Error::RsaSignatureIsInvalid);
    // }

    // Check quote fields
    if quote.header.version != QUOTE_VERSION_V3 {
        return Err(Error::UnsupportedDCAPQuoteVersion);
    }
    // We only support ECDSA256 with P256 curve
    if quote.header.attestation_key_type != ATTESTATION_KEY_TYPE_ECDSA256_WITH_P256_CURVE {
        return Err(Error::UnsupportedDCAPAttestationKeyType);
    }

    // Extract Auth data from quote
    let AuthData::V3(auth_data) = quote.auth_data else {
        return Err(Error::UnsupportedQuoteAuthData);
    };
    let certification_data = auth_data.certification_data;

    // We only support 5 -Concatenated PCK Cert Chain (PEM formatted).
    if certification_data.cert_type != 5 {
        return Err(Error::UnsupportedDCAPPckCertFormat);
    }

    // TODO: need port ring
    // let certification_certs = extract_certs(&certification_data.body.data)?;
    // if certification_certs.len() < 2 {
    //     return Err(Error::CertificateChainIsTooShort);
    // }
    // // Check certification_data
    // let leaf_cert: webpki::EndEntityCert =
    //     webpki::EndEntityCert::try_from(&certification_certs[0])
    //         .map_err(|_| Error::LeafCertificateParsingError)?;
    // let intermediate_certs = &certification_certs[1..];
    // verify_certificate_chain(&leaf_cert, intermediate_certs, now_in_milli)?;

    // Check QE signature
    // TODO: need port ring
    // let asn1_signature = encode_as_der(&auth_data.qe_report_signature)?;
    // if leaf_cert
    //     .verify_signature(webpki::ECDSA_P256_SHA256, &auth_data.qe_report, &asn1_signature)
    //     .is_err()
    // {
    //     return Err(Error::RsaSignatureIsInvalid);
    // }

    // Extract QE report from quote
    let mut qe_report = auth_data.qe_report.as_slice();
    let qe_report =
        EnclaveReport::decode(&mut qe_report).map_err(|_err| Error::CodecError)?;

    // Check QE hash
    let mut qe_hash_data = [0u8; QE_HASH_DATA_BYTE_LEN];
    qe_hash_data[0..ATTESTATION_KEY_LEN].copy_from_slice(&auth_data.ecdsa_attestation_key);
    qe_hash_data[ATTESTATION_KEY_LEN..].copy_from_slice(&auth_data.qe_auth_data.data);
    // TODO: need port ring
    // let qe_hash = ring::digest::digest(&ring::digest::SHA256, &qe_hash_data);
    // if qe_hash.as_ref() != &qe_report.report_data[0..32] {
    //     return Err(Error::QEReportHashMismatch);
    // }

    // Check signature from auth data
    // TODO: need port ring
    // let mut pub_key = [0x04u8; 65]; //Prepend 0x04 to specify uncompressed format
    // pub_key[1..].copy_from_slice(&auth_data.ecdsa_attestation_key);
    // let peer_public_key =
    //     ring::signature::UnparsedPublicKey::new(&ring::signature::ECDSA_P256_SHA256_FIXED, pub_key);
    // peer_public_key
    //     .verify(
    //         &raw_quote[..(HEADER_BYTE_LEN + ENCLAVE_REPORT_BYTE_LEN)],
    //         &auth_data.ecdsa_signature,
    //     )
    //     .map_err(|_| Error::IsvEnclaveReportSignatureIsInvalid)?;

    // Extract information from the quote

    // TODO: need port ring
    // let extension_section = get_intel_extension(&certification_certs[0])?;
    // let cpu_svn = get_cpu_svn(&extension_section)?;
    // let pce_svn = get_pce_svn(&extension_section)?;
    // let fmspc = get_fmspc(&extension_section)?;

    let cpu_svn = qe_report.cpu_svn;
    let pce_svn = quote.header.pce_svn;

    // TODO: need port ring
    // let tcb_fmspc = hex::decode(&tcb_info.fmspc).map_err(|_| Error::CodecError)?;
    // if fmspc != tcb_fmspc[..] {
    //     return Err(Error::FmspcMismatch);
    // }

    // TCB status and advisory ids
    let mut tcb_status = "Unknown".to_owned();
    let mut advisory_ids = Vec::<String>::new();
    for tcb_level in &tcb_info.tcb_levels {
        if pce_svn >= tcb_level.tcb.pce_svn {
            if cpu_svn.iter().zip(&tcb_level.tcb.components).any(|(a, b)| a < &b.svn) {
                continue
            }

            tcb_status = tcb_level.tcb_status.clone();
            tcb_level
                .advisory_ids
                .iter()
                .for_each(|id| advisory_ids.push(id.clone()));

            break;
        }
    }

    let mr_enclave = quote.report.mr_enclave.to_vec();
    let mr_signer = quote.report.mr_signer.to_vec();
    let isv_prod_id = quote.report.isv_prod_id;
    let isv_svn = quote.report.isv_svn;

    Ok((
        quote.report.report_data.to_vec(),
        mr_enclave,
        mr_signer,
        isv_prod_id,
        isv_svn,
        tcb_status.to_string(),
        advisory_ids,
    ))
}
