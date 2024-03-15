#[doc = "Register `GMAC_MMC_RXTCPERRFRM` reader"]
pub type R = crate::R<GmacMmcRxtcperrfrmSpec>;
#[doc = "Register `GMAC_MMC_RXTCPERRFRM` writer"]
pub type W = crate::W<GmacMmcRxtcperrfrmSpec>;
#[doc = "Field `RXTCP_ERR_FRMS` reader - Number of good IP datagrams whose TCP payload has a checksum error."]
pub type RxtcpErrFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXTCP_ERR_FRMS` writer - Number of good IP datagrams whose TCP payload has a checksum error."]
pub type RxtcpErrFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose TCP payload has a checksum error."]
    #[inline(always)]
    pub fn rxtcp_err_frms(&self) -> RxtcpErrFrmsR {
        RxtcpErrFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose TCP payload has a checksum error."]
    #[inline(always)]
    #[must_use]
    pub fn rxtcp_err_frms(&mut self) -> RxtcpErrFrmsW<GmacMmcRxtcperrfrmSpec> {
        RxtcpErrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX TCP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxtcperrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxtcperrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxtcperrfrmSpec;
impl crate::RegisterSpec for GmacMmcRxtcperrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxtcperrfrm::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxtcperrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxtcperrfrm::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxtcperrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXTCPERRFRM to value 0"]
impl crate::Resettable for GmacMmcRxtcperrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
