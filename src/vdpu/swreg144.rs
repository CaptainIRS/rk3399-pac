#[doc = "Register `SWREG144` reader"]
pub type R = crate::R<Swreg144Spec>;
#[doc = "Register `SWREG144` writer"]
pub type W = crate::W<Swreg144Spec>;
#[doc = "Field `MFR_REG24` reader - multi format reuse register24 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 16\n\n\\[27:24\\]
: tab2:code words of length 15\n\n\\[23:20\\]
: tab2:code words of length 14\n\n\\[19:16\\]
: tab2:code words of length 13\n\n\\[15:12\\]
: tab2:code words of length 12\n\n\\[11:8\\]
: tab2:code words of length 11\n\n\\[6:4\\]
: tab2:code words of length 10\n\n\\[1:0\\]
: tab2:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 36st coef of scan read index\n\n\\[23:18\\]
: 37st coef of scan read index\n\n\\[17:12\\]
: 38st coef of scan read index\n\n\\[11:6\\]
: 39st coef of scan read index\n\n\\[5:0\\]
: 40st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+5 start address"]
pub type MfrReg24R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG24` writer - multi format reuse register24 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 16\n\n\\[27:24\\]
: tab2:code words of length 15\n\n\\[23:20\\]
: tab2:code words of length 14\n\n\\[19:16\\]
: tab2:code words of length 13\n\n\\[15:12\\]
: tab2:code words of length 12\n\n\\[11:8\\]
: tab2:code words of length 11\n\n\\[6:4\\]
: tab2:code words of length 10\n\n\\[1:0\\]
: tab2:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 36st coef of scan read index\n\n\\[23:18\\]
: 37st coef of scan read index\n\n\\[17:12\\]
: 38st coef of scan read index\n\n\\[11:6\\]
: 39st coef of scan read index\n\n\\[5:0\\]
: 40st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+5 start address"]
pub type MfrReg24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register24 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 16\n\n\\[27:24\\]
: tab2:code words of length 15\n\n\\[23:20\\]
: tab2:code words of length 14\n\n\\[19:16\\]
: tab2:code words of length 13\n\n\\[15:12\\]
: tab2:code words of length 12\n\n\\[11:8\\]
: tab2:code words of length 11\n\n\\[6:4\\]
: tab2:code words of length 10\n\n\\[1:0\\]
: tab2:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 36st coef of scan read index\n\n\\[23:18\\]
: 37st coef of scan read index\n\n\\[17:12\\]
: 38st coef of scan read index\n\n\\[11:6\\]
: 39st coef of scan read index\n\n\\[5:0\\]
: 40st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+5 start address"]
    #[inline(always)]
    pub fn mfr_reg24(&self) -> MfrReg24R {
        MfrReg24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register24 except h264\n\nJPEG:\n\n\\[31:28\\]
: tab2:code words of length 16\n\n\\[27:24\\]
: tab2:code words of length 15\n\n\\[23:20\\]
: tab2:code words of length 14\n\n\\[19:16\\]
: tab2:code words of length 13\n\n\\[15:12\\]
: tab2:code words of length 12\n\n\\[11:8\\]
: tab2:code words of length 11\n\n\\[6:4\\]
: tab2:code words of length 10\n\n\\[1:0\\]
: tab2:code words of length 9\n\nVP6:\n\n\\[29:24\\]
: 36st coef of scan read index\n\n\\[23:18\\]
: 37st coef of scan read index\n\n\\[17:12\\]
: 38st coef of scan read index\n\n\\[11:6\\]
: 39st coef of scan read index\n\n\\[5:0\\]
: 40st coef of scan read index\n\nVP7\n\n\\[31:2\\]
: DCT stream MB row 2,2n+5 start address"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg24(&mut self) -> MfrReg24W<Swreg144Spec> {
        MfrReg24W::new(self, 0)
    }
}
#[doc = "multi format reuse register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg144::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg144::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg144Spec;
impl crate::RegisterSpec for Swreg144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg144::R`](R) reader structure"]
impl crate::Readable for Swreg144Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg144::W`](W) writer structure"]
impl crate::Writable for Swreg144Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG144 to value 0"]
impl crate::Resettable for Swreg144Spec {
    const RESET_VALUE: u32 = 0;
}
