#[doc = "Register `IDLE_TH` reader"]
pub type R = crate::R<IdleThSpec>;
#[doc = "Register `IDLE_TH` writer"]
pub type W = crate::W<IdleThSpec>;
#[doc = "Field `IDLE_TH` reader - Idle counter threshold in standby mode"]
pub type IdleThR = crate::FieldReader<u16>;
#[doc = "Field `IDLE_TH` writer - Idle counter threshold in standby mode"]
pub type IdleThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Idle counter threshold in standby mode"]
    #[inline(always)]
    pub fn idle_th(&self) -> IdleThR {
        IdleThR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Idle counter threshold in standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn idle_th(&mut self) -> IdleThW<IdleThSpec> {
        IdleThW::new(self, 0)
    }
}
#[doc = "DDR Controller LP Interface Idle Threshold in standby mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_th::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_th::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleThSpec;
impl crate::RegisterSpec for IdleThSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_th::R`](R) reader structure"]
impl crate::Readable for IdleThSpec {}
#[doc = "`write(|w| ..)` method takes [`idle_th::W`](W) writer structure"]
impl crate::Writable for IdleThSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLE_TH to value 0"]
impl crate::Resettable for IdleThSpec {
    const RESET_VALUE: u32 = 0;
}
