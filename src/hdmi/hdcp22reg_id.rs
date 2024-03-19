#[doc = "Register `HDCP22REG_ID` reader"]
pub type R = crate::R<Hdcp22regIdSpec>;
#[doc = "Field `HDCP22_EXTERNALIF` reader - Indicates that External HDCP 2.2 interface is\n\npresent.\n\nReset Value: (HTX_HDCP22_EXTERNAL_NONE== 1) ? 1 : 0"]
pub type Hdcp22ExternalifR = crate::BitReader;
#[doc = "Field `HDCP22_3RDPARTY` reader - Indicates that External HDCP 2.2 interface is present\n\nand 3rd party HDCP 2.2 module is connected to this\n\ninterface.\n\nReset Value: (HTX_HDCP22_EXTERNAL_ELLP== 1) ? 1 : 0"]
pub type Hdcp22_3rdpartyR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Indicates that External HDCP 2.2 interface is\n\npresent.\n\nReset Value: (HTX_HDCP22_EXTERNAL_NONE== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdcp22_externalif(&self) -> Hdcp22ExternalifR {
        Hdcp22ExternalifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that External HDCP 2.2 interface is present\n\nand 3rd party HDCP 2.2 module is connected to this\n\ninterface.\n\nReset Value: (HTX_HDCP22_EXTERNAL_ELLP== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn hdcp22_3rdparty(&self) -> Hdcp22_3rdpartyR {
        Hdcp22_3rdpartyR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "HDCP 2.2 Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp22reg_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hdcp22regIdSpec;
impl crate::RegisterSpec for Hdcp22regIdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp22reg_id::R`](R) reader structure"]
impl crate::Readable for Hdcp22regIdSpec {}
#[doc = "`reset()` method sets HDCP22REG_ID to value 0"]
impl crate::Resettable for Hdcp22regIdSpec {
    const RESET_VALUE: u8 = 0;
}
