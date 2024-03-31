#[doc = "Register `n_INTSTATUS` reader"]
pub type R = crate::R<NIntstatusSpec>;
#[doc = "Register `n_INTSTATUS` writer"]
pub type W = crate::W<NIntstatusSpec>;
#[doc = "Field `INT_PD` reader - This register contains the interrupt status for timer n.\n\nWrite 1 to this register will clear the interrupt."]
pub type IntPdR = crate::BitReader;
#[doc = "Field `INT_PD` writer - This register contains the interrupt status for timer n.\n\nWrite 1 to this register will clear the interrupt."]
pub type IntPdW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register contains the interrupt status for timer n.\n\nWrite 1 to this register will clear the interrupt."]
    #[inline(always)]
    pub fn int_pd(&self) -> IntPdR {
        IntPdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register contains the interrupt status for timer n.\n\nWrite 1 to this register will clear the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn int_pd(&mut self) -> IntPdW<NIntstatusSpec> {
        IntPdW::new(self, 0)
    }
}
#[doc = "Timer Interrupt Stauts Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`n_intstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`n_intstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NIntstatusSpec;
impl crate::RegisterSpec for NIntstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`n_intstatus::R`](R) reader structure"]
impl crate::Readable for NIntstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`n_intstatus::W`](W) writer structure"]
impl crate::Writable for NIntstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets n_INTSTATUS to value 0"]
impl crate::Resettable for NIntstatusSpec {
    const RESET_VALUE: u32 = 0;
}
