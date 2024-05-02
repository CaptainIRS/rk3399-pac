#[doc = "Register `FLASH_MAXP` reader"]
pub type R = crate::R<FlashMaxpSpec>;
#[doc = "Register `FLASH_MAXP` writer"]
pub type W = crate::W<FlashMaxpSpec>;
#[doc = "Field `fl_maxp` reader - maximum period value for flash or preflash\n\nmax. flash/preflash period = 214 * (fl_maxp + 1) / clk_isp fl_maxp = (max_period * clk_isp) / 214 - 1"]
pub type FlMaxpR = crate::FieldReader<u16>;
#[doc = "Field `fl_maxp` writer - maximum period value for flash or preflash\n\nmax. flash/preflash period = 214 * (fl_maxp + 1) / clk_isp fl_maxp = (max_period * clk_isp) / 214 - 1"]
pub type FlMaxpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - maximum period value for flash or preflash\n\nmax. flash/preflash period = 214 * (fl_maxp + 1) / clk_isp fl_maxp = (max_period * clk_isp) / 214 - 1"]
    #[inline(always)]
    pub fn fl_maxp(&self) -> FlMaxpR {
        FlMaxpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - maximum period value for flash or preflash\n\nmax. flash/preflash period = 214 * (fl_maxp + 1) / clk_isp fl_maxp = (max_period * clk_isp) / 214 - 1"]
    #[inline(always)]
    #[must_use]
    pub fn fl_maxp(&mut self) -> FlMaxpW<FlashMaxpSpec> {
        FlMaxpW::new(self, 0)
    }
}
#[doc = "Maximum value for flash or preflash\n\nNote: Example: \n\n\n\nfl_maxp = (10s * 100MHz) / (16384) – 1 = 61034 \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_maxp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_maxp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashMaxpSpec;
impl crate::RegisterSpec for FlashMaxpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_maxp::R`](R) reader structure"]
impl crate::Readable for FlashMaxpSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_maxp::W`](W) writer structure"]
impl crate::Writable for FlashMaxpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_MAXP to value 0"]
impl crate::Resettable for FlashMaxpSpec {
    const RESET_VALUE: u32 = 0;
}
