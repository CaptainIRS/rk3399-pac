#[doc = "Register `SWREG41_RLCWRITE_BASE` reader"]
pub type R = crate::R<Swreg41RlcwriteBaseSpec>;
#[doc = "Register `SWREG41_RLCWRITE_BASE` writer"]
pub type W = crate::W<Swreg41RlcwriteBaseSpec>;
#[doc = "Field `SW_RLCWRITE_BASE` reader - the base address of rlcwrite\n\nthe base address of rlcwrite(the address should 64bit align)\n\ncabac output write to this rlcwrite base address when\n\nsw_rlc_mode_direct_write in swreg2_sysctrl is valid"]
pub type SwRlcwriteBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_RLCWRITE_BASE` writer - the base address of rlcwrite\n\nthe base address of rlcwrite(the address should 64bit align)\n\ncabac output write to this rlcwrite base address when\n\nsw_rlc_mode_direct_write in swreg2_sysctrl is valid"]
pub type SwRlcwriteBaseW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - the base address of rlcwrite\n\nthe base address of rlcwrite(the address should 64bit align)\n\ncabac output write to this rlcwrite base address when\n\nsw_rlc_mode_direct_write in swreg2_sysctrl is valid"]
    #[inline(always)]
    pub fn sw_rlcwrite_base(&self) -> SwRlcwriteBaseR {
        SwRlcwriteBaseR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - the base address of rlcwrite\n\nthe base address of rlcwrite(the address should 64bit align)\n\ncabac output write to this rlcwrite base address when\n\nsw_rlc_mode_direct_write in swreg2_sysctrl is valid"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rlcwrite_base(&mut self) -> SwRlcwriteBaseW<Swreg41RlcwriteBaseSpec> {
        SwRlcwriteBaseW::new(self, 3)
    }
}
#[doc = "the base address or rlcwrite base addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg41_rlcwrite_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg41_rlcwrite_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg41RlcwriteBaseSpec;
impl crate::RegisterSpec for Swreg41RlcwriteBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg41_rlcwrite_base::R`](R) reader structure"]
impl crate::Readable for Swreg41RlcwriteBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg41_rlcwrite_base::W`](W) writer structure"]
impl crate::Writable for Swreg41RlcwriteBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG41_RLCWRITE_BASE to value 0"]
impl crate::Resettable for Swreg41RlcwriteBaseSpec {
    const RESET_VALUE: u32 = 0;
}
