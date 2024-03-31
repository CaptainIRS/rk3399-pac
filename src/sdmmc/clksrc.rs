#[doc = "Register `CLKSRC` reader"]
pub type R = crate::R<ClksrcSpec>;
#[doc = "Register `CLKSRC` writer"]
pub type W = crate::W<ClksrcSpec>;
#[doc = "Field `CLK_SOURCE` reader - Clock divider source for up to 16 SD cards supported. Each card\n\nhas two bits assigned to it. For example, bits\\[1:0\\]
assigned for\n\ncard-0, which maps and internally routes clock divider\\[3:0\\]\n\noutputs to cclk_out\\[15:0\\]
pins, depending on bit value.\n\n00: Clock divider 0\n\nThe cclk_out is always from clock divider 0, and this register is\n\nnot implemented."]
pub type ClkSourceR = crate::FieldReader;
#[doc = "Field `CLK_SOURCE` writer - Clock divider source for up to 16 SD cards supported. Each card\n\nhas two bits assigned to it. For example, bits\\[1:0\\]
assigned for\n\ncard-0, which maps and internally routes clock divider\\[3:0\\]\n\noutputs to cclk_out\\[15:0\\]
pins, depending on bit value.\n\n00: Clock divider 0\n\nThe cclk_out is always from clock divider 0, and this register is\n\nnot implemented."]
pub type ClkSourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Clock divider source for up to 16 SD cards supported. Each card\n\nhas two bits assigned to it. For example, bits\\[1:0\\]
assigned for\n\ncard-0, which maps and internally routes clock divider\\[3:0\\]\n\noutputs to cclk_out\\[15:0\\]
pins, depending on bit value.\n\n00: Clock divider 0\n\nThe cclk_out is always from clock divider 0, and this register is\n\nnot implemented."]
    #[inline(always)]
    pub fn clk_source(&self) -> ClkSourceR {
        ClkSourceR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock divider source for up to 16 SD cards supported. Each card\n\nhas two bits assigned to it. For example, bits\\[1:0\\]
assigned for\n\ncard-0, which maps and internally routes clock divider\\[3:0\\]\n\noutputs to cclk_out\\[15:0\\]
pins, depending on bit value.\n\n00: Clock divider 0\n\nThe cclk_out is always from clock divider 0, and this register is\n\nnot implemented."]
    #[inline(always)]
    #[must_use]
    pub fn clk_source(&mut self) -> ClkSourceW<ClksrcSpec> {
        ClkSourceW::new(self, 0)
    }
}
#[doc = "SD clock source register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClksrcSpec;
impl crate::RegisterSpec for ClksrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksrc::R`](R) reader structure"]
impl crate::Readable for ClksrcSpec {}
#[doc = "`write(|w| ..)` method takes [`clksrc::W`](W) writer structure"]
impl crate::Writable for ClksrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSRC to value 0"]
impl crate::Resettable for ClksrcSpec {
    const RESET_VALUE: u32 = 0;
}
