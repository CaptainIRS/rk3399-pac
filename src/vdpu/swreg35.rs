#[doc = "Register `SWREG35` reader"]
pub type R = crate::R<Swreg35Spec>;
#[doc = "Register `SWREG35` writer"]
pub type W = crate::W<Swreg35Spec>;
#[doc = "Field `SW_PP_OUTW` reader - the pp output width\n\n(output width = 8*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inw-8))"]
pub type SwPpOutwR = crate::FieldReader<u16>;
#[doc = "Field `SW_PP_OUTW` writer - the pp output width\n\n(output width = 8*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inw-8))"]
pub type SwPpOutwW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_PP_OUTW_EXT` reader - the extension width of pp output"]
pub type SwPpOutwExtR = crate::BitReader;
#[doc = "Field `SW_PP_OUTW_EXT` writer - the extension width of pp output"]
pub type SwPpOutwExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_PP_OUTH` reader - (output width = 2*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inh-8))"]
pub type SwPpOuthR = crate::FieldReader<u16>;
#[doc = "Field `SW_PP_OUTH` writer - (output width = 2*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inh-8))"]
pub type SwPpOuthW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_PP_OUTH_EXT` reader - the extension height of pp output"]
pub type SwPpOuthExtR = crate::BitReader;
#[doc = "Field `SW_PP_OUTH_EXT` writer - the extension height of pp output"]
pub type SwPpOuthExtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - the pp output width\n\n(output width = 8*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inw-8))"]
    #[inline(always)]
    pub fn sw_pp_outw(&self) -> SwPpOutwR {
        SwPpOutwR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - the extension width of pp output"]
    #[inline(always)]
    pub fn sw_pp_outw_ext(&self) -> SwPpOutwExtR {
        SwPpOutwExtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:26 - (output width = 2*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inh-8))"]
    #[inline(always)]
    pub fn sw_pp_outh(&self) -> SwPpOuthR {
        SwPpOuthR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - the extension height of pp output"]
    #[inline(always)]
    pub fn sw_pp_outh_ext(&self) -> SwPpOuthExtR {
        SwPpOuthExtR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - the pp output width\n\n(output width = 8*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inw-8))"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_outw(&mut self) -> SwPpOutwW<Swreg35Spec> {
        SwPpOutwW::new(self, 0)
    }
    #[doc = "Bit 11 - the extension width of pp output"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_outw_ext(&mut self) -> SwPpOutwExtW<Swreg35Spec> {
        SwPpOutwExtW::new(self, 11)
    }
    #[doc = "Bits 16:26 - (output width = 2*n (n=1,2,......) || output width\n\n=(configurationed Pixel Accurate PP output configuration)*n )\n\n&amp;&amp;\n\n( pp output width &lt; 1920 || pp output width&lt; 3*(sw_pp_inh-8))"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_outh(&mut self) -> SwPpOuthW<Swreg35Spec> {
        SwPpOuthW::new(self, 16)
    }
    #[doc = "Bit 27 - the extension height of pp output"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_outh_ext(&mut self) -> SwPpOuthExtW<Swreg35Spec> {
        SwPpOuthExtW::new(self, 27)
    }
}
#[doc = "PP output pic size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg35Spec;
impl crate::RegisterSpec for Swreg35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg35::R`](R) reader structure"]
impl crate::Readable for Swreg35Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg35::W`](W) writer structure"]
impl crate::Writable for Swreg35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG35 to value 0"]
impl crate::Resettable for Swreg35Spec {
    const RESET_VALUE: u32 = 0;
}
