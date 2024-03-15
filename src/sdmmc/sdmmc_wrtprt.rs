#[doc = "Register `SDMMC_WRTPRT` reader"]
pub type R = crate::R<SdmmcWrtprtSpec>;
#[doc = "Register `SDMMC_WRTPRT` writer"]
pub type W = crate::W<SdmmcWrtprtSpec>;
#[doc = "Field `WRITE_PROTECT` reader - Value on card_write_prt input port. 1 represents write protection."]
pub type WriteProtectR = crate::BitReader;
#[doc = "Field `WRITE_PROTECT` writer - Value on card_write_prt input port. 1 represents write protection."]
pub type WriteProtectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Value on card_write_prt input port. 1 represents write protection."]
    #[inline(always)]
    pub fn write_protect(&self) -> WriteProtectR {
        WriteProtectR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Value on card_write_prt input port. 1 represents write protection."]
    #[inline(always)]
    #[must_use]
    pub fn write_protect(&mut self) -> WriteProtectW<SdmmcWrtprtSpec> {
        WriteProtectW::new(self, 0)
    }
}
#[doc = "Write-protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_wrtprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_wrtprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcWrtprtSpec;
impl crate::RegisterSpec for SdmmcWrtprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_wrtprt::R`](R) reader structure"]
impl crate::Readable for SdmmcWrtprtSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_wrtprt::W`](W) writer structure"]
impl crate::Writable for SdmmcWrtprtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_WRTPRT to value 0"]
impl crate::Resettable for SdmmcWrtprtSpec {
    const RESET_VALUE: u32 = 0;
}
