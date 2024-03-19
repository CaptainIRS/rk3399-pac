#[doc = "Register `PCIE_LM_SHADOW_UR_ERROR` reader"]
pub type R = crate::R<PcieLmShadowUrErrorSpec>;
#[doc = "Register `PCIE_LM_SHADOW_UR_ERROR` writer"]
pub type W = crate::W<PcieLmShadowUrErrorSpec>;
#[doc = "Field `P_UR_ERR` writer - Posted UR Error \\[P_UR_ERR\\]\n\nIf this bit is set, the corresponding\n\nposted UR error bits will be set in\n\nthe AER and device status registers\n\nof the target function."]
pub type PUrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NP_UR_ERR` writer - Non Posted Error \\[NP_UR_ERR\\]\n\nIf this bit is set, the corresponding\n\nnon-posted UR error bits will be set\n\nin the AER and device status\n\nregisters of the target function."]
pub type NpUrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Posted UR Error \\[P_UR_ERR\\]\n\nIf this bit is set, the corresponding\n\nposted UR error bits will be set in\n\nthe AER and device status registers\n\nof the target function."]
    #[inline(always)]
    #[must_use]
    pub fn p_ur_err(&mut self) -> PUrErrW<PcieLmShadowUrErrorSpec> {
        PUrErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Non Posted Error \\[NP_UR_ERR\\]\n\nIf this bit is set, the corresponding\n\nnon-posted UR error bits will be set\n\nin the AER and device status\n\nregisters of the target function."]
    #[inline(always)]
    #[must_use]
    pub fn np_ur_err(&mut self) -> NpUrErrW<PcieLmShadowUrErrorSpec> {
        NpUrErrW::new(self, 1)
    }
}
#[doc = "Shadow Register UR Error\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_shadow_ur_error::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_shadow_ur_error::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmShadowUrErrorSpec;
impl crate::RegisterSpec for PcieLmShadowUrErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_shadow_ur_error::R`](R) reader structure"]
impl crate::Readable for PcieLmShadowUrErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_shadow_ur_error::W`](W) writer structure"]
impl crate::Writable for PcieLmShadowUrErrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_SHADOW_UR_ERROR to value 0"]
impl crate::Resettable for PcieLmShadowUrErrorSpec {
    const RESET_VALUE: u32 = 0;
}
