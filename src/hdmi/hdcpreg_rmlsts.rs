#[doc = "Register `HDCPREG_RMLSTS` reader"]
pub type R = crate::R<HdcpregRmlstsSpec>;
#[doc = "Field `IDPK_DATA_INDEX` reader - Current Device Private Key being written plus one. Position 0 is occupied by the AKSV."]
pub type IdpkDataIndexR = crate::FieldReader;
#[doc = "Field `IDPK_WR_OK_STS` reader - When high (1'b1), it indicates that a DPK write is allowed."]
pub type IdpkWrOkStsR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Current Device Private Key being written plus one. Position 0 is occupied by the AKSV."]
    #[inline(always)]
    pub fn idpk_data_index(&self) -> IdpkDataIndexR {
        IdpkDataIndexR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - When high (1'b1), it indicates that a DPK write is allowed."]
    #[inline(always)]
    pub fn idpk_wr_ok_sts(&self) -> IdpkWrOkStsR {
        IdpkWrOkStsR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Current Device Private Key being written plus one. Position 0 is occupied by the AKSV.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_rmlsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregRmlstsSpec;
impl crate::RegisterSpec for HdcpregRmlstsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_rmlsts::R`](R) reader structure"]
impl crate::Readable for HdcpregRmlstsSpec {}
#[doc = "`reset()` method sets HDCPREG_RMLSTS to value 0"]
impl crate::Resettable for HdcpregRmlstsSpec {
    const RESET_VALUE: u8 = 0;
}
