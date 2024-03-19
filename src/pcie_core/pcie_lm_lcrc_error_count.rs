#[doc = "Register `PCIE_LM_LCRC_ERROR_COUNT` reader"]
pub type R = crate::R<PcieLmLcrcErrorCountSpec>;
#[doc = "Register `PCIE_LM_LCRC_ERROR_COUNT` writer"]
pub type W = crate::W<PcieLmLcrcErrorCountSpec>;
#[doc = "Field `LEC` reader - LCRC Eror Count \\[LEC\\]\n\nNumber of TLPs received with LCRC\n\nerrors."]
pub type LecR = crate::FieldReader<u16>;
#[doc = "Field `LEC` writer - LCRC Eror Count \\[LEC\\]\n\nNumber of TLPs received with LCRC\n\nerrors."]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R11` reader - Reserved \\[R11\\]\n\nReserved"]
pub type R11R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LCRC Eror Count \\[LEC\\]\n\nNumber of TLPs received with LCRC\n\nerrors."]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R11\\]\n\nReserved"]
    #[inline(always)]
    pub fn r11(&self) -> R11R {
        R11R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LCRC Eror Count \\[LEC\\]\n\nNumber of TLPs received with LCRC\n\nerrors."]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LecW<PcieLmLcrcErrorCountSpec> {
        LecW::new(self, 0)
    }
}
#[doc = "LCRC Error Count Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_lcrc_error_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_lcrc_error_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmLcrcErrorCountSpec;
impl crate::RegisterSpec for PcieLmLcrcErrorCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_lcrc_error_count::R`](R) reader structure"]
impl crate::Readable for PcieLmLcrcErrorCountSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_lcrc_error_count::W`](W) writer structure"]
impl crate::Writable for PcieLmLcrcErrorCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff;
}
#[doc = "`reset()` method sets PCIE_LM_LCRC_ERROR_COUNT to value 0"]
impl crate::Resettable for PcieLmLcrcErrorCountSpec {
    const RESET_VALUE: u32 = 0;
}
