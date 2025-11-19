/**
 * Mock for lightweight-charts library
 */

// Mock 系列对象
const createMockSeries = () => ({
  setData: jest.fn(),
  applyOptions: jest.fn(),
  update: jest.fn(),
  setMarkers: jest.fn(),
  priceScale: jest.fn(() => ({
    applyOptions: jest.fn(),
  })),
});

export const createChart = jest.fn(() => ({
  panes: jest.fn(() => [
    {
      addSeries: jest.fn(() => createMockSeries()),
    },
  ]),
  // 直接添加系列的方法（用于多系列图表）
  addLineSeries: jest.fn(() => createMockSeries()),
  addAreaSeries: jest.fn(() => createMockSeries()),
  applyOptions: jest.fn(),
  timeScale: jest.fn(() => ({
    fitContent: jest.fn(),
    scrollToPosition: jest.fn(),
    setVisibleRange: jest.fn(),
  })),
  priceScale: jest.fn(() => ({
    applyOptions: jest.fn(),
  })),
  remove: jest.fn(),
  resize: jest.fn(),
}));

export const LineSeries = { seriesType: 'Line' };
export const AreaSeries = { seriesType: 'Area' };

export const ColorType = {
  Solid: 'solid',
  VerticalGradient: 'gradient',
};

export const CrosshairMode = {
  Normal: 0,
  Magnet: 1,
  Hidden: 2,
  MagnetOHLC: 3,
};

export const LineStyle = {
  Solid: 0,
  Dotted: 1,
  Dashed: 2,
  LargeDashed: 3,
  SparseDotted: 4,
};

export const PriceScaleMode = {
  Normal: 0,
  Logarithmic: 1,
  Percentage: 2,
  IndexedTo100: 3,
};
