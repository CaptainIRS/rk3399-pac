#[doc = "Register `SWREG145` reader"]
pub type R = crate::R<Swreg145Spec>;
#[doc = "Register `SWREG145` writer"]
pub type W = crate::W<Swreg145Spec>;
#[doc = "Field `MFR_REG25` reader - multi format reuse register25 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 8\n\n\\[27:24\\]
: tab3:code words of length 7\n\n\\[23:20\\]
: tab3:code words of length 6\n\n\\[19:16\\]
: tab3:code words of length 5\n\n\\[15:12\\]
: tab3:code words of length 4\n\n\\[11:8\\]
: tab3:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab3:code words of length 1\n\nVP6/VP7:\n\n\\[31:2\\]
: ctrl data stream start address"]
pub type MfrReg25R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG25` writer - multi format reuse register25 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 8\n\n\\[27:24\\]
: tab3:code words of length 7\n\n\\[23:20\\]
: tab3:code words of length 6\n\n\\[19:16\\]
: tab3:code words of length 5\n\n\\[15:12\\]
: tab3:code words of length 4\n\n\\[11:8\\]
: tab3:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab3:code words of length 1\n\nVP6/VP7:\n\n\\[31:2\\]
: ctrl data stream start address"]
pub type MfrReg25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register25 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 8\n\n\\[27:24\\]
: tab3:code words of length 7\n\n\\[23:20\\]
: tab3:code words of length 6\n\n\\[19:16\\]
: tab3:code words of length 5\n\n\\[15:12\\]
: tab3:code words of length 4\n\n\\[11:8\\]
: tab3:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab3:code words of length 1\n\nVP6/VP7:\n\n\\[31:2\\]
: ctrl data stream start address"]
    #[inline(always)]
    pub fn mfr_reg25(&self) -> MfrReg25R {
        MfrReg25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register25 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab3:code words of length 8\n\n\\[27:24\\]
: tab3:code words of length 7\n\n\\[23:20\\]
: tab3:code words of length 6\n\n\\[19:16\\]
: tab3:code words of length 5\n\n\\[15:12\\]
: tab3:code words of length 4\n\n\\[11:8\\]
: tab3:code words of length 3\n\n\\[6:4\\]
: tab2:code words of length 2\n\n\\[1:0\\]
: tab3:code words of length 1\n\nVP6/VP7:\n\n\\[31:2\\]
: ctrl data stream start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg25(&mut self) -> MfrReg25W<Swreg145Spec> {
        MfrReg25W::new(self, 0)
    }
}
#[doc = "multi format reuse register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg145::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg145::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg145Spec;
impl crate::RegisterSpec for Swreg145Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg145::R`](R) reader structure"]
impl crate::Readable for Swreg145Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg145::W`](W) writer structure"]
impl crate::Writable for Swreg145Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG145 to value 0"]
impl crate::Resettable for Swreg145Spec {
    const RESET_VALUE: u32 = 0;
}
