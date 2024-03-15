#[doc = "Register `ACTIVE_LINE_CFG_H` reader"]
pub type R = crate::R<ActiveLineCfgHSpec>;
#[doc = "Register `ACTIVE_LINE_CFG_H` writer"]
pub type W = crate::W<ActiveLineCfgHSpec>;
#[doc = "Field `ACTIVE_LINE_CFG_H` reader - ACTIVE_LINE_CFG is used to specify the number of active lines in each frame. This register is ACTIVE_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type ActiveLineCfgHR = crate::FieldReader;
#[doc = "Field `ACTIVE_LINE_CFG_H` writer - ACTIVE_LINE_CFG is used to specify the number of active lines in each frame. This register is ACTIVE_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type ActiveLineCfgHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ACTIVE_LINE_CFG is used to specify the number of active lines in each frame. This register is ACTIVE_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn active_line_cfg_h(&self) -> ActiveLineCfgHR {
        ActiveLineCfgHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ACTIVE_LINE_CFG is used to specify the number of active lines in each frame. This register is ACTIVE_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn active_line_cfg_h(&mut self) -> ActiveLineCfgHW<ActiveLineCfgHSpec> {
        ActiveLineCfgHW::new(self, 0)
    }
}
#[doc = "Active Line High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`active_line_cfg_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`active_line_cfg_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveLineCfgHSpec;
impl crate::RegisterSpec for ActiveLineCfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active_line_cfg_h::R`](R) reader structure"]
impl crate::Readable for ActiveLineCfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`active_line_cfg_h::W`](W) writer structure"]
impl crate::Writable for ActiveLineCfgHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets ACTIVE_LINE_CFG_H to value 0"]
impl crate::Resettable for ActiveLineCfgHSpec {
    const RESET_VALUE: u32 = 0;
}
