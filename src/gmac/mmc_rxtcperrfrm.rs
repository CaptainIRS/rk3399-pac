#[doc = "Register `MMC_RXTCPERRFRM` reader"]
pub type R = crate::R<MmcRxtcperrfrmSpec>;
#[doc = "Register `MMC_RXTCPERRFRM` writer"]
pub type W = crate::W<MmcRxtcperrfrmSpec>;
#[doc = "Field `RXTCP_ERR_FRMS` reader - Number of good IP datagrams whose TCP payload has a\n\nchecksum error."]
pub type RxtcpErrFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXTCP_ERR_FRMS` writer - Number of good IP datagrams whose TCP payload has a\n\nchecksum error."]
pub type RxtcpErrFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose TCP payload has a\n\nchecksum error."]
    #[inline(always)]
    pub fn rxtcp_err_frms(&self) -> RxtcpErrFrmsR {
        RxtcpErrFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose TCP payload has a\n\nchecksum error."]
    #[inline(always)]
    #[must_use]
    pub fn rxtcp_err_frms(&mut self) -> RxtcpErrFrmsW<MmcRxtcperrfrmSpec> {
        RxtcpErrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX TCP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxtcperrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxtcperrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxtcperrfrmSpec;
impl crate::RegisterSpec for MmcRxtcperrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxtcperrfrm::R`](R) reader structure"]
impl crate::Readable for MmcRxtcperrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxtcperrfrm::W`](W) writer structure"]
impl crate::Writable for MmcRxtcperrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXTCPERRFRM to value 0"]
impl crate::Resettable for MmcRxtcperrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
