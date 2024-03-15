#[doc = "Register `GMAC_MMC_RXTCPERROCT` reader"]
pub type R = crate::R<GmacMmcRxtcperroctSpec>;
#[doc = "Register `GMAC_MMC_RXTCPERROCT` writer"]
pub type W = crate::W<GmacMmcRxtcperroctSpec>;
#[doc = "Field `RXTCP_ERR_OCTETS` reader - Number of bytes received in a TCP segment with checksum errors."]
pub type RxtcpErrOctetsR = crate::FieldReader<u32>;
#[doc = "Field `RXTCP_ERR_OCTETS` writer - Number of bytes received in a TCP segment with checksum errors."]
pub type RxtcpErrOctetsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in a TCP segment with checksum errors."]
    #[inline(always)]
    pub fn rxtcp_err_octets(&self) -> RxtcpErrOctetsR {
        RxtcpErrOctetsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in a TCP segment with checksum errors."]
    #[inline(always)]
    #[must_use]
    pub fn rxtcp_err_octets(&mut self) -> RxtcpErrOctetsW<GmacMmcRxtcperroctSpec> {
        RxtcpErrOctetsW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET TCP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxtcperroct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxtcperroct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxtcperroctSpec;
impl crate::RegisterSpec for GmacMmcRxtcperroctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxtcperroct::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxtcperroctSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxtcperroct::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxtcperroctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXTCPERROCT to value 0"]
impl crate::Resettable for GmacMmcRxtcperroctSpec {
    const RESET_VALUE: u32 = 0;
}
