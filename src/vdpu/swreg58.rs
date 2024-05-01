#[doc = "Register `SWREG58` reader"]
pub type R = crate::R<Swreg58Spec>;
#[doc = "Register `SWREG58` writer"]
pub type W = crate::W<Swreg58Spec>;
#[doc = "Field `SW_SOFT_RST` reader - the soft reset for decoder or pp or encoder\n\nwrite 1 to reset,and it will auto reset to 0 after one cycle"]
pub type SwSoftRstR = crate::BitReader;
#[doc = "Field `SW_SOFT_RST` writer - the soft reset for decoder or pp or encoder\n\nwrite 1 to reset,and it will auto reset to 0 after one cycle"]
pub type SwSoftRstW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - the soft reset for decoder or pp or encoder\n\nwrite 1 to reset,and it will auto reset to 0 after one cycle"]
    #[inline(always)]
    pub fn sw_soft_rst(&self) -> SwSoftRstR {
        SwSoftRstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the soft reset for decoder or pp or encoder\n\nwrite 1 to reset,and it will auto reset to 0 after one cycle"]
    #[inline(always)]
    #[must_use]
    pub fn sw_soft_rst(&mut self) -> SwSoftRstW<Swreg58Spec> {
        SwSoftRstW::new(self, 0)
    }
}
#[doc = "soft reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg58Spec;
impl crate::RegisterSpec for Swreg58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg58::R`](R) reader structure"]
impl crate::Readable for Swreg58Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg58::W`](W) writer structure"]
impl crate::Writable for Swreg58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets SWREG58 to value 0"]
impl crate::Resettable for Swreg58Spec {
    const RESET_VALUE: u32 = 0;
}
