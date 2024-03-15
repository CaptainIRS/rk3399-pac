#[doc = "Register `CIC_CG_WAIT_TH` reader"]
pub type R = crate::R<CicCgWaitThSpec>;
#[doc = "Register `CIC_CG_WAIT_TH` writer"]
pub type W = crate::W<CicCgWaitThSpec>;
#[doc = "Field `CG_WAIT_TH` reader - Clock gating wait counter threshold in standby mode"]
pub type CgWaitThR = crate::FieldReader<u16>;
#[doc = "Field `CG_WAIT_TH` writer - Clock gating wait counter threshold in standby mode"]
pub type CgWaitThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CG_EXIT_TH` reader - Clock gating exit counter threshold in standby mode"]
pub type CgExitThR = crate::FieldReader<u16>;
#[doc = "Field `CG_EXIT_TH` writer - Clock gating exit counter threshold in standby mode"]
pub type CgExitThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock gating wait counter threshold in standby mode"]
    #[inline(always)]
    pub fn cg_wait_th(&self) -> CgWaitThR {
        CgWaitThR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Clock gating exit counter threshold in standby mode"]
    #[inline(always)]
    pub fn cg_exit_th(&self) -> CgExitThR {
        CgExitThR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock gating wait counter threshold in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn cg_wait_th(&mut self) -> CgWaitThW<CicCgWaitThSpec> {
        CgWaitThW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Clock gating exit counter threshold in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn cg_exit_th(&mut self) -> CgExitThW<CicCgWaitThSpec> {
        CgExitThW::new(self, 16)
    }
}
#[doc = "DDR Controller LP Interface CG Wait Threshold in standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cic_cg_wait_th::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cic_cg_wait_th::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicCgWaitThSpec;
impl crate::RegisterSpec for CicCgWaitThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cic_cg_wait_th::R`](R) reader structure"]
impl crate::Readable for CicCgWaitThSpec {}
#[doc = "`write(|w| ..)` method takes [`cic_cg_wait_th::W`](W) writer structure"]
impl crate::Writable for CicCgWaitThSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIC_CG_WAIT_TH to value 0"]
impl crate::Resettable for CicCgWaitThSpec {
    const RESET_VALUE: u32 = 0;
}
