#[doc = "Register `DDR_DENALI_CTL_228` reader"]
pub type R = crate::R<DdrDenaliCtl228Spec>;
#[doc = "Register `DDR_DENALI_CTL_228` writer"]
pub type W = crate::W<DdrDenaliCtl228Spec>;
#[doc = "Field `WRLVL_CS_MAP` reader - Defines the chip select map for write leveling operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
pub type WrlvlCsMapR = crate::FieldReader;
#[doc = "Field `WRLVL_CS_MAP` writer - Defines the chip select map for write leveling operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
pub type WrlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRLVL_ERROR_STATUS` reader - Holds the error associated with the write level error interrupt. Bit (0) set indicates a TDFI_WRLVL_MAX parameter violation and bit (1) set indicates a TDFI_WRLVL_RESP parameter violation."]
pub type WrlvlErrorStatusR = crate::FieldReader;
#[doc = "Field `WRLVL_NORM_THRESHOLD_F0` reader - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type WrlvlNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_NORM_THRESHOLD_F0` writer - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type WrlvlNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Defines the chip select map for write leveling operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
    #[inline(always)]
    pub fn wrlvl_cs_map(&self) -> WrlvlCsMapR {
        WrlvlCsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Holds the error associated with the write level error interrupt. Bit (0) set indicates a TDFI_WRLVL_MAX parameter violation and bit (1) set indicates a TDFI_WRLVL_RESP parameter violation."]
    #[inline(always)]
    pub fn wrlvl_error_status(&self) -> WrlvlErrorStatusR {
        WrlvlErrorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn wrlvl_norm_threshold_f0(&self) -> WrlvlNormThresholdF0R {
        WrlvlNormThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines the chip select map for write leveling operations. Bit (0) controls cs0, bit (1) controls cs1, etc. Set each bit to 1 to enable chip for write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_cs_map(&mut self) -> WrlvlCsMapW<DdrDenaliCtl228Spec> {
        WrlvlCsMapW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_norm_threshold_f0(&mut self) -> WrlvlNormThresholdF0W<DdrDenaliCtl228Spec> {
        WrlvlNormThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_228::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_228::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl228Spec;
impl crate::RegisterSpec for DdrDenaliCtl228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_228::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl228Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_228::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl228Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_228 to value 0"]
impl crate::Resettable for DdrDenaliCtl228Spec {
    const RESET_VALUE: u32 = 0;
}
