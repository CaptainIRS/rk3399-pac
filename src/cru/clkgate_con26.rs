#[doc = "Register `CLKGATE_CON26` reader"]
pub type R = crate::R<ClkgateCon26Spec>;
#[doc = "Register `CLKGATE_CON26` writer"]
pub type W = crate::W<ClkgateCon26Spec>;
#[doc = "Field `CLK_TIMER0_EN` reader - clk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer0EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER0_EN` writer - clk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER1_EN` reader - clk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer1EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER1_EN` writer - clk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER2_EN` reader - clk_timer2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer2EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER2_EN` writer - clk_timer2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER3_EN` reader - clk_timer3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer3EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER3_EN` writer - clk_timer3 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER4_EN` reader - clk_timer4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer4EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER4_EN` writer - clk_timer4 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER5_EN` reader - clk_timer5 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer5EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER5_EN` writer - clk_timer5 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER6_EN` reader - clk_timer6 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer6EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER6_EN` writer - clk_timer6 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER7_EN` reader - clk_timer7 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer7EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER7_EN` writer - clk_timer7 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER8_EN` reader - clk_timer8 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer8EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER8_EN` writer - clk_timer8 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER9_EN` reader - clk_timer9 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer9EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER9_EN` writer - clk_timer9 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer9EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER10_EN` reader - clk_timer10 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer10EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER10_EN` writer - clk_timer10 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer10EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TIMER11_EN` reader - clk_timer11 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer11EnR = crate::BitReader;
#[doc = "Field `CLK_TIMER11_EN` writer - clk_timer11 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTimer11EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer0_en(&self) -> ClkTimer0EnR {
        ClkTimer0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer1_en(&self) -> ClkTimer1EnR {
        ClkTimer1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_timer2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer2_en(&self) -> ClkTimer2EnR {
        ClkTimer2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_timer3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer3_en(&self) -> ClkTimer3EnR {
        ClkTimer3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_timer4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer4_en(&self) -> ClkTimer4EnR {
        ClkTimer4EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_timer5 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer5_en(&self) -> ClkTimer5EnR {
        ClkTimer5EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_timer6 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer6_en(&self) -> ClkTimer6EnR {
        ClkTimer6EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_timer7 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer7_en(&self) -> ClkTimer7EnR {
        ClkTimer7EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_timer8 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer8_en(&self) -> ClkTimer8EnR {
        ClkTimer8EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_timer9 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer9_en(&self) -> ClkTimer9EnR {
        ClkTimer9EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_timer10 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer10_en(&self) -> ClkTimer10EnR {
        ClkTimer10EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_timer11 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_timer11_en(&self) -> ClkTimer11EnR {
        ClkTimer11EnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_timer0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer0_en(&mut self) -> ClkTimer0EnW<ClkgateCon26Spec> {
        ClkTimer0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_timer1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer1_en(&mut self) -> ClkTimer1EnW<ClkgateCon26Spec> {
        ClkTimer1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_timer2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer2_en(&mut self) -> ClkTimer2EnW<ClkgateCon26Spec> {
        ClkTimer2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_timer3 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer3_en(&mut self) -> ClkTimer3EnW<ClkgateCon26Spec> {
        ClkTimer3EnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_timer4 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer4_en(&mut self) -> ClkTimer4EnW<ClkgateCon26Spec> {
        ClkTimer4EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_timer5 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer5_en(&mut self) -> ClkTimer5EnW<ClkgateCon26Spec> {
        ClkTimer5EnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_timer6 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer6_en(&mut self) -> ClkTimer6EnW<ClkgateCon26Spec> {
        ClkTimer6EnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_timer7 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer7_en(&mut self) -> ClkTimer7EnW<ClkgateCon26Spec> {
        ClkTimer7EnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_timer8 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer8_en(&mut self) -> ClkTimer8EnW<ClkgateCon26Spec> {
        ClkTimer8EnW::new(self, 8)
    }
    #[doc = "Bit 9 - clk_timer9 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer9_en(&mut self) -> ClkTimer9EnW<ClkgateCon26Spec> {
        ClkTimer9EnW::new(self, 9)
    }
    #[doc = "Bit 10 - clk_timer10 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer10_en(&mut self) -> ClkTimer10EnW<ClkgateCon26Spec> {
        ClkTimer10EnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_timer11 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer11_en(&mut self) -> ClkTimer11EnW<ClkgateCon26Spec> {
        ClkTimer11EnW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon26Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon26Spec;
impl crate::RegisterSpec for ClkgateCon26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con26::R`](R) reader structure"]
impl crate::Readable for ClkgateCon26Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con26::W`](W) writer structure"]
impl crate::Writable for ClkgateCon26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON26 to value 0"]
impl crate::Resettable for ClkgateCon26Spec {
    const RESET_VALUE: u32 = 0;
}
