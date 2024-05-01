#[doc = "Register `SWREG155` reader"]
pub type R = crate::R<Swreg155Spec>;
#[doc = "Register `SWREG155` writer"]
pub type W = crate::W<Swreg155Spec>;
#[doc = "Field `MFR_REG35` reader - multi format reuse register35 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 2,tap1\n\n\\[21:12\\]
: prediction filter with set 2,tap2\n\n\\[11:2\\]
: prediction filter with set 2,tap3"]
pub type MfrReg35R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG35` writer - multi format reuse register35 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 2,tap1\n\n\\[21:12\\]
: prediction filter with set 2,tap2\n\n\\[11:2\\]
: prediction filter with set 2,tap3"]
pub type MfrReg35W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register35 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 2,tap1\n\n\\[21:12\\]
: prediction filter with set 2,tap2\n\n\\[11:2\\]
: prediction filter with set 2,tap3"]
    #[inline(always)]
    pub fn mfr_reg35(&self) -> MfrReg35R {
        MfrReg35R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register35 except h264\n\nVP6/VP7:\n\n\\[31:22\\]
: prediction filter with set 2,tap1\n\n\\[21:12\\]
: prediction filter with set 2,tap2\n\n\\[11:2\\]
: prediction filter with set 2,tap3"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg35(&mut self) -> MfrReg35W<Swreg155Spec> {
        MfrReg35W::new(self, 0)
    }
}
#[doc = "multi format reuse register35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg155::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg155::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg155Spec;
impl crate::RegisterSpec for Swreg155Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg155::R`](R) reader structure"]
impl crate::Readable for Swreg155Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg155::W`](W) writer structure"]
impl crate::Writable for Swreg155Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG155 to value 0"]
impl crate::Resettable for Swreg155Spec {
    const RESET_VALUE: u32 = 0;
}
