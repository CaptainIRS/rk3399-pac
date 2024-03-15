#[doc = "Register `TOTAL_LINE_CFG_H` reader"]
pub type R = crate::R<TotalLineCfgHSpec>;
#[doc = "Register `TOTAL_LINE_CFG_H` writer"]
pub type W = crate::W<TotalLineCfgHSpec>;
#[doc = "Field `TOTAL_LINE_CFG_H` reader - TOTAL_LINE_CFG is used to specify the number of lines in each frame. This register is TOTAL_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When Video BIST_EN is enabled, this bit must be configured right to generate right video format."]
pub type TotalLineCfgHR = crate::FieldReader;
#[doc = "Field `TOTAL_LINE_CFG_H` writer - TOTAL_LINE_CFG is used to specify the number of lines in each frame. This register is TOTAL_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When Video BIST_EN is enabled, this bit must be configured right to generate right video format."]
pub type TotalLineCfgHW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TOTAL_LINE_CFG is used to specify the number of lines in each frame. This register is TOTAL_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When Video BIST_EN is enabled, this bit must be configured right to generate right video format."]
    #[inline(always)]
    pub fn total_line_cfg_h(&self) -> TotalLineCfgHR {
        TotalLineCfgHR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TOTAL_LINE_CFG is used to specify the number of lines in each frame. This register is TOTAL_LINE_CFG \\[11:8\\]. When F_SEL is 1, this value is sent in main stream attribute packet. When Video BIST_EN is enabled, this bit must be configured right to generate right video format."]
    #[inline(always)]
    #[must_use]
    pub fn total_line_cfg_h(&mut self) -> TotalLineCfgHW<TotalLineCfgHSpec> {
        TotalLineCfgHW::new(self, 0)
    }
}
#[doc = "Total Line High Byte Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`total_line_cfg_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`total_line_cfg_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TotalLineCfgHSpec;
impl crate::RegisterSpec for TotalLineCfgHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`total_line_cfg_h::R`](R) reader structure"]
impl crate::Readable for TotalLineCfgHSpec {}
#[doc = "`write(|w| ..)` method takes [`total_line_cfg_h::W`](W) writer structure"]
impl crate::Writable for TotalLineCfgHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets TOTAL_LINE_CFG_H to value 0"]
impl crate::Resettable for TotalLineCfgHSpec {
    const RESET_VALUE: u32 = 0;
}
