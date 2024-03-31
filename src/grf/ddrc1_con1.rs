#[doc = "Register `DDRC1_CON1` reader"]
pub type R = crate::R<Ddrc1Con1Spec>;
#[doc = "Register `DDRC1_CON1` writer"]
pub type W = crate::W<Ddrc1Con1Spec>;
#[doc = "Field `CLK_DDRC1_EN_STDBY` reader - bit control of clk_ddrc1_en_stdby"]
pub type ClkDdrc1EnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDRC1_EN_STDBY` writer - bit control of clk_ddrc1_en_stdby"]
pub type ClkDdrc1EnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY_CTRL1_EN_STDBY` reader - bit control of clk_ddrphy_ctrl1_en_stdby"]
pub type ClkDdrphyCtrl1EnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY_CTRL1_EN_STDBY` writer - bit control of clk_ddrphy_ctrl1_en_stdby"]
pub type ClkDdrphyCtrl1EnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRPHY1_EN_STDBY` reader - bit control of clk_ddrphy1_en_stdby"]
pub type ClkDdrphy1EnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDRPHY1_EN_STDBY` writer - bit control of clk_ddrphy1_en_stdby"]
pub type ClkDdrphy1EnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDR1_MSCH_EN_STDBY` reader - bit control of clk_ddr1_msch_en_stdby"]
pub type ClkDdr1MschEnStdbyR = crate::BitReader;
#[doc = "Field `CLK_DDR1_MSCH_EN_STDBY` writer - bit control of clk_ddr1_msch_en_stdby"]
pub type ClkDdr1MschEnStdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENALI1_COMMAND_PRIORITY` reader - bit control of denali1_command_priority"]
pub type Denali1CommandPriorityR = crate::FieldReader;
#[doc = "Field `DENALI1_COMMAND_PRIORITY` writer - bit control of denali1_command_priority"]
pub type Denali1CommandPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4 - bit control of clk_ddrc1_en_stdby"]
    #[inline(always)]
    pub fn clk_ddrc1_en_stdby(&self) -> ClkDdrc1EnStdbyR {
        ClkDdrc1EnStdbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - bit control of clk_ddrphy_ctrl1_en_stdby"]
    #[inline(always)]
    pub fn clk_ddrphy_ctrl1_en_stdby(&self) -> ClkDdrphyCtrl1EnStdbyR {
        ClkDdrphyCtrl1EnStdbyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bit control of clk_ddrphy1_en_stdby"]
    #[inline(always)]
    pub fn clk_ddrphy1_en_stdby(&self) -> ClkDdrphy1EnStdbyR {
        ClkDdrphy1EnStdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - bit control of clk_ddr1_msch_en_stdby"]
    #[inline(always)]
    pub fn clk_ddr1_msch_en_stdby(&self) -> ClkDdr1MschEnStdbyR {
        ClkDdr1MschEnStdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - bit control of denali1_command_priority"]
    #[inline(always)]
    pub fn denali1_command_priority(&self) -> Denali1CommandPriorityR {
        Denali1CommandPriorityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - bit control of clk_ddrc1_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc1_en_stdby(&mut self) -> ClkDdrc1EnStdbyW<Ddrc1Con1Spec> {
        ClkDdrc1EnStdbyW::new(self, 4)
    }
    #[doc = "Bit 5 - bit control of clk_ddrphy_ctrl1_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy_ctrl1_en_stdby(&mut self) -> ClkDdrphyCtrl1EnStdbyW<Ddrc1Con1Spec> {
        ClkDdrphyCtrl1EnStdbyW::new(self, 5)
    }
    #[doc = "Bit 6 - bit control of clk_ddrphy1_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrphy1_en_stdby(&mut self) -> ClkDdrphy1EnStdbyW<Ddrc1Con1Spec> {
        ClkDdrphy1EnStdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - bit control of clk_ddr1_msch_en_stdby"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddr1_msch_en_stdby(&mut self) -> ClkDdr1MschEnStdbyW<Ddrc1Con1Spec> {
        ClkDdr1MschEnStdbyW::new(self, 7)
    }
    #[doc = "Bits 8:9 - bit control of denali1_command_priority"]
    #[inline(always)]
    #[must_use]
    pub fn denali1_command_priority(&mut self) -> Denali1CommandPriorityW<Ddrc1Con1Spec> {
        Denali1CommandPriorityW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Ddrc1Con1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "ddrc1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrc1_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrc1_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddrc1Con1Spec;
impl crate::RegisterSpec for Ddrc1Con1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrc1_con1::R`](R) reader structure"]
impl crate::Readable for Ddrc1Con1Spec {}
#[doc = "`write(|w| ..)` method takes [`ddrc1_con1::W`](W) writer structure"]
impl crate::Writable for Ddrc1Con1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRC1_CON1 to value 0"]
impl crate::Resettable for Ddrc1Con1Spec {
    const RESET_VALUE: u32 = 0;
}
