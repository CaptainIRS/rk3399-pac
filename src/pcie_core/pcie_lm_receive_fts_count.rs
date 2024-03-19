#[doc = "Register `PCIE_LM_RECEIVE_FTS_COUNT` reader"]
pub type R = crate::R<PcieLmReceiveFtsCountSpec>;
#[doc = "Field `RFC5S` reader - Received FTS Count for 5GT/s Speed \\[RFC5S\\]
FTS count received from the other side during link training for use at the 5 GT/s link speed. The core transmits this many FTS sequences while exiting the L0S state, when operating at the 5 GT/s speed."]
pub type Rfc5sR = crate::FieldReader;
#[doc = "Field `R7` reader - Reserved \\[R7\\]
Reserved"]
pub type R7R = crate::FieldReader;
#[doc = "Field `R72` reader - Reserved \\[R72\\]
Reserved"]
pub type R72R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Received FTS Count for 5GT/s Speed \\[RFC5S\\]
FTS count received from the other side during link training for use at the 5 GT/s link speed. The core transmits this many FTS sequences while exiting the L0S state, when operating at the 5 GT/s speed."]
    #[inline(always)]
    pub fn rfc5s(&self) -> Rfc5sR {
        Rfc5sR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved \\[R7\\]
Reserved"]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Reserved \\[R72\\]
Reserved"]
    #[inline(always)]
    pub fn r72(&self) -> R72R {
        R72R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Receive FTS Count Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_fts_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmReceiveFtsCountSpec;
impl crate::RegisterSpec for PcieLmReceiveFtsCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_receive_fts_count::R`](R) reader structure"]
impl crate::Readable for PcieLmReceiveFtsCountSpec {}
#[doc = "`reset()` method sets PCIE_LM_RECEIVE_FTS_COUNT to value 0"]
impl crate::Resettable for PcieLmReceiveFtsCountSpec {
    const RESET_VALUE: u32 = 0;
}
