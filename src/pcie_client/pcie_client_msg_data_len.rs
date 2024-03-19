#[doc = "Register `PCIE_CLIENT_MSG_DATA_LEN` reader"]
pub type R = crate::R<PcieClientMsgDataLenSpec>;
#[doc = "Field `LENGTH1` reader - Length1\n\nLength1, record the recently received message length.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Length1R = crate::FieldReader;
#[doc = "Field `LENGTH2` reader - Length2\n\nLength2, record the 2nd recently received message length.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Length2R = crate::FieldReader;
#[doc = "Field `LENGTH3` reader - Length3\n\nLength3, record the 3rd recently received message length.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Length3R = crate::FieldReader;
#[doc = "Field `LENGTH4` reader - Length4\n\nLength4, record the 4th recently received message length.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Length4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Length1\n\nLength1, record the recently received message length."]
    #[inline(always)]
    pub fn length1(&self) -> Length1R {
        Length1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Length2\n\nLength2, record the 2nd recently received message length."]
    #[inline(always)]
    pub fn length2(&self) -> Length2R {
        Length2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Length3\n\nLength3, record the 3rd recently received message length."]
    #[inline(always)]
    pub fn length3(&self) -> Length3R {
        Length3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Length4\n\nLength4, record the 4th recently received message length."]
    #[inline(always)]
    pub fn length4(&self) -> Length4R {
        Length4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Message data length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_msg_data_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientMsgDataLenSpec;
impl crate::RegisterSpec for PcieClientMsgDataLenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_msg_data_len::R`](R) reader structure"]
impl crate::Readable for PcieClientMsgDataLenSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_MSG_DATA_LEN to value 0"]
impl crate::Resettable for PcieClientMsgDataLenSpec {
    const RESET_VALUE: u32 = 0;
}
